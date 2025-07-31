// https://www.w3.org/TR/xpath-functions-31/#node-functions

use ahash::HashSet;
use ahash::HashSetExt;
use xee_xpath_macros::xpath_fn;

use crate::atomic;
use crate::error;
use crate::function::StaticFunctionDescription;
use crate::interpreter::Interpreter;
use crate::wrap_xpath_fn;

#[xpath_fn("fn:name($arg as node()?) as xs:string", context_first)]
fn name(interpreter: &Interpreter, arg: Option<xot::Node>) -> error::Result<String> {
    Ok(if let Some(node) = arg {
        let name = interpreter.xot().node_name(node);
        if let Some(name) = name {
            interpreter.xot().full_name(node, name)?
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    })
}

#[xpath_fn("fn:local-name($arg as node()?) as xs:string", context_first)]
fn local_name(interpreter: &Interpreter, arg: Option<xot::Node>) -> String {
    if let Some(arg) = arg {
        let name = interpreter.xot().node_name(arg);
        if let Some(name) = name {
            interpreter.xot().local_name_str(name).to_string()
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    }
}

#[xpath_fn("fn:namespace-uri($arg as node()?) as xs:anyURI", context_first)]
fn namespace_uri(interpreter: &Interpreter, arg: Option<xot::Node>) -> atomic::Atomic {
    let uri = if let Some(arg) = arg {
        let name = interpreter.xot().node_name(arg);
        if let Some(name) = name {
            interpreter.xot().uri_str(name).to_string()
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };
    atomic::Atomic::String(atomic::StringType::AnyURI, uri.into())
}

#[xpath_fn(
    "fn:lang($testlang as xs:string?, $node as node()) as xs:boolean",
    context_last
)]
fn lang(interpreter: &Interpreter, testlang: Option<&str>, node: xot::Node) -> error::Result<bool> {
    let xot = interpreter.xot();
    let lang_name = xot.name_ns("lang", xot.xml_namespace());
    let test_lang = testlang.unwrap_or("");
    if let Some(lang_name) = lang_name {
        let mut lang = None;
        for node in xot.ancestors(node) {
            if let Some(attribute) = xot.get_attribute(node, lang_name) {
                lang = Some(attribute);
                break;
            }
        }
        if let Some(lang) = lang {
            let lang = lang.to_lowercase();
            let test_lang = test_lang.to_lowercase();
            if lang == test_lang {
                Ok(true)
            } else {
                Ok(lang.starts_with(&test_lang) && lang.chars().nth(test_lang.len()) == Some('-'))
            }
        } else {
            // no lang attribute found
            Ok(false)
        }
    } else {
        // no lang name known anywhere so cannot occur
        Ok(false)
    }
}

#[xpath_fn("fn:root($arg as node()?) as node()?", context_first)]
fn root(interpreter: &Interpreter, arg: Option<xot::Node>) -> Option<xot::Node> {
    arg.map(|arg| interpreter.xot().root(arg))
}

#[xpath_fn("fn:has-children($node as node()?) as xs:boolean", context_first)]
fn has_children(interpreter: &Interpreter, node: Option<xot::Node>) -> bool {
    if let Some(node) = node {
        interpreter.xot().first_child(node).is_some()
    } else {
        false
    }
}

#[xpath_fn("fn:innermost($nodes as node()*) as node()*")]
fn innermost(
    interpreter: &Interpreter,
    nodes: impl Iterator<Item = error::Result<xot::Node>>,
) -> error::Result<Vec<xot::Node>> {
    let nodes: Vec<xot::Node> = nodes.collect::<error::Result<_>>()?;
    // get sequence of ancestors
    let mut ancestors = HashSet::new();
    for node in nodes.iter() {
        let mut parent_node = *node;
        // insert all parents into ancestors
        while let Some(parent) = interpreter.xot().parent(parent_node) {
            ancestors.insert(parent);
            parent_node = parent;
        }
    }
    // now find all nodes that are not in ancestors
    let mut innermost = Vec::new();
    for node in nodes {
        if !ancestors.contains(&node) {
            innermost.push(node);
        }
    }
    Ok(innermost)
}

#[xpath_fn("fn:outermost($nodes as node()*) as node()*")]
fn outermost(
    interpreter: &Interpreter,
    nodes: impl Iterator<Item = error::Result<xot::Node>>,
) -> error::Result<Vec<xot::Node>> {
    let nodes: Vec<xot::Node> = nodes.collect::<error::Result<_>>()?;
    let node_set = nodes.iter().collect::<HashSet<_>>();
    // now find all nodes that don't have an ancestor in the set
    let mut outermost = Vec::new();
    'outer: for node in nodes.iter() {
        let mut parent_node = *node;
        // if we find an ancestor in node_set, then we don't add this node
        while let Some(parent) = interpreter.xot().parent(parent_node) {
            if node_set.contains(&parent) {
                continue 'outer;
            }
            parent_node = parent;
        }
        outermost.push(*node);
    }
    Ok(outermost)
}

#[xpath_fn("fn:path($arg as node()?) as xs:string?", context_first)]
fn path(interpreter: &Interpreter, arg: Option<xot::Node>) -> Option<String> {
    if let Some(node) = arg {
        if interpreter.xot().is_document(node) {
            Some("/".to_string())
        } else {
            Some(path_helper(node, interpreter.xot()))
        }
    } else {
        None
    }
}

fn path_helper(node: xot::Node, xot: &xot::Xot) -> String {
    match xot.value(node) {
        xot::Value::Document => "".to_string(),
        xot::Value::Element(e) => {
            let name = e.name();
            let (local, ns) = xot.name_ns_str(name);
            let position = position_by_type(node, xot, |child, xot| {
                if let Some(element) = xot.element(child) {
                    element.name() == name
                } else {
                    false
                }
            });
            let path = parent_path(node, xot);
            format!("{}/Q{{{}}}{}[{}]", path, ns, local, position)
        }
        xot::Value::Text(_) => {
            let position = position_by_type(node, xot, |child, xot| xot.is_text(child));
            format!("{}/text()[{}]", parent_path(node, xot), position)
        }
        xot::Value::Comment(_) => {
            let position = position_by_type(node, xot, |child, xot| xot.is_comment(child));
            format!("{}/comment()[{}]", parent_path(node, xot), position)
        }
        xot::Value::ProcessingInstruction(p) => {
            let target = p.target();
            let position = position_by_type(node, xot, |child, xot| {
                if let Some(processing_instruction) = xot.processing_instruction(child) {
                    processing_instruction.target() == target
                } else {
                    false
                }
            });
            let (local, _) = xot.name_ns_str(target);

            format!(
                "{}/processing-instruction({})[{}]",
                parent_path(node, xot),
                local,
                position
            )
        }
        xot::Value::Attribute(attribute) => {
            let name = attribute.name();
            let (local, ns) = xot.name_ns_str(name);
            let s = if ns.is_empty() {
                local.to_string()
            } else {
                format!("Q{{{}}}{}", ns, local)
            };
            format!("{}/@{}", parent_path(node, xot), s)
        }
        xot::Value::Namespace(n) => {
            let prefix = n.prefix();
            let s = if xot.empty_prefix() != prefix {
                xot.prefix_str(prefix)
            } else {
                "*[Q{http://www.w3.org/2005/xpath-functions}local-name()=\"\"]"
            };
            format!("{}/namespace::{}", parent_path(node, xot), s)
        }
    }
}

fn parent_path(node: xot::Node, xot: &xot::Xot) -> String {
    if let Some(parent) = xot.parent(node) {
        path_helper(parent, xot)
    } else {
        "Q{http://www.w3.org/2005/xpath-functions}root()".to_string()
    }
}

fn position_by_type(
    node: xot::Node,
    xot: &xot::Xot,
    is_type: impl Fn(xot::Node, &xot::Xot) -> bool,
) -> usize {
    let mut position = 1;
    let parent = xot.parent(node);
    if let Some(parent) = parent {
        for child in xot.children(parent) {
            if child == node {
                return position;
            }
            if is_type(child, xot) {
                position += 1;
            }
        }
        unreachable!()
    } else {
        1
    }
}

pub(crate) fn static_function_descriptions() -> Vec<StaticFunctionDescription> {
    vec![
        wrap_xpath_fn!(name),
        wrap_xpath_fn!(local_name),
        wrap_xpath_fn!(namespace_uri),
        wrap_xpath_fn!(lang),
        wrap_xpath_fn!(root),
        wrap_xpath_fn!(has_children),
        wrap_xpath_fn!(innermost),
        wrap_xpath_fn!(outermost),
        wrap_xpath_fn!(path),
    ]
}
