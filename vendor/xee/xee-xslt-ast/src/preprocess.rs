use xot::{Element, Node, NodeEdge, Xot};

#[derive(Debug)]
enum Error {
    Xot(xot::Error),
    Internal,
}

impl From<xot::Error> for Error {
    fn from(error: xot::Error) -> Self {
        Self::Xot(error)
    }
}

// conditional element inclusion and pre-processing shadow-attributes
fn preprocess(xot: &mut Xot, top: Node) -> Result<(), Error> {
    let xsl_ns = xot.add_namespace("http://www.w3.org/1999/XSL/Transform");
    let instruction_use_when = xot.add_name("use-when");
    let xsl_use_when = xot.add_name_ns("use-when", xsl_ns);
    let in_xsl_ns = |element: &Element| {
        let name = element.name();
        let namespace = xot.namespace_for_name(name);
        namespace == xsl_ns
    };

    let mut to_remove = Vec::new();
    let mut to_remove_attribute = Vec::new();
    let mut non_shadow_names = Vec::new();
    let mut shadow_attributes = Vec::new();

    let mut stack = Vec::new();

    for edge in xot.traverse(top) {
        match edge {
            NodeEdge::Start(node) => {
                // if we're in a removed element, we don't need to
                // do anything as it's all going to disappear
                if let Some(in_removed) = stack.last() {
                    if *in_removed {
                        continue;
                    }
                }
                if let Some(element) = xot.element(node) {
                    // record all shadow attributes to process
                    for key in xot.attributes(node).keys() {
                        let (name, ns) = xot.name_ns_str(key);
                        if ns.is_empty() && name.starts_with('_') {
                            let non_shadow_name = name[1..].to_string();
                            non_shadow_names.push(non_shadow_name);
                            shadow_attributes.push((node, key));
                        }
                    }
                    // TODO: use-when itself can be a shadow attribute
                    let use_when = if in_xsl_ns(element) {
                        instruction_use_when
                    } else {
                        xsl_use_when
                    };
                    if let Some(value) = xot.attributes(node).get(use_when) {
                        // TODO: replace with real xpath evaluation
                        if value == "false()" {
                            to_remove.push(node);
                            stack.push(true);
                        } else {
                            to_remove_attribute.push((node, use_when));
                            stack.push(false);
                        }
                    } else {
                        stack.push(false);
                    }
                }
            }
            NodeEdge::End(_) => {
                stack.pop();
            }
        }
    }

    for non_shadow_name in non_shadow_names {
        xot.add_name(&non_shadow_name);
    }

    for (node, shadow_name) in shadow_attributes {
        let (name, _) = xot.name_ns_str(shadow_name);
        let non_shadow_name = &name[1..];
        let non_shadow_name = xot.name(non_shadow_name).ok_or(Error::Internal)?;
        let value = xot
            .attributes(node)
            .get(shadow_name)
            .ok_or(Error::Internal)?
            .to_string();
        // TODO execute xpath
        let mut attributes = xot.attributes_mut(node);
        attributes.insert(non_shadow_name, value);
        attributes.remove(shadow_name);
    }

    for node in to_remove {
        xot.remove(node)?;
    }

    for (node, name) in to_remove_attribute {
        xot.attributes_mut(node).remove(name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_when_without_content() {
        let mut xot = Xot::new();
        let doc = xot.parse(r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"><xsl:if use-when="false()"></xsl:if></xsl:transform>"#).unwrap();
        preprocess(&mut xot, doc).unwrap();
        assert_eq!(
            xot.to_string(doc).unwrap(),
            r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"/>"#
        );
    }

    #[test]
    fn test_use_when_with_content() {
        let mut xot = Xot::new();
        let doc = xot.parse(r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"><xsl:if use-when="false()"><p/></xsl:if></xsl:transform>"#).unwrap();
        preprocess(&mut xot, doc).unwrap();
        assert_eq!(
            xot.to_string(doc).unwrap(),
            r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"/>"#
        );
    }

    #[test]
    fn test_nested_use_when() {
        let mut xot = Xot::new();
        let doc = xot.parse(r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"><xsl:if use-when="false()"><xsl:if use-when="false()"><p/></xsl:if></xsl:if></xsl:transform>"#).unwrap();
        preprocess(&mut xot, doc).unwrap();
        assert_eq!(
            xot.to_string(doc).unwrap(),
            r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"/>"#
        );
    }

    #[test]
    fn test_xsl_use_when_on_literal_element() {
        let mut xot = Xot::new();
        let doc = xot.parse(r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"><foo xsl:use-when="false()"/></xsl:transform>"#).unwrap();
        preprocess(&mut xot, doc).unwrap();
        assert_eq!(
            xot.to_string(doc).unwrap(),
            r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"/>"#
        );
    }

    #[test]
    fn test_xsl_use_when_on_instruction_is_ignored() {
        let mut xot = Xot::new();
        let doc = xot.parse(r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"><xsl:if xsl:use-when="false()"/></xsl:transform>"#).unwrap();
        preprocess(&mut xot, doc).unwrap();
        assert_eq!(
            xot.to_string(doc).unwrap(),
            r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"><xsl:if xsl:use-when="false()"/></xsl:transform>"#
        );
    }

    #[test]
    fn test_unprefixed_use_when_on_literal_element_is_ignored() {
        let mut xot = Xot::new();
        let doc = xot.parse(r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"><p use-when="false()"/></xsl:transform>"#).unwrap();
        preprocess(&mut xot, doc).unwrap();
        assert_eq!(
            xot.to_string(doc).unwrap(),
            r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"><p use-when="false()"/></xsl:transform>"#
        );
    }

    #[test]
    fn test_use_when_is_stripped_when_true() {
        let mut xot = Xot::new();
        let doc = xot.parse(r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"><xsl:if use-when="true()"/></xsl:transform>"#).unwrap();
        preprocess(&mut xot, doc).unwrap();
        assert_eq!(
            xot.to_string(doc).unwrap(),
            r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"><xsl:if/></xsl:transform>"#
        );
    }

    #[test]
    fn test_shadow_attribute_no_operation() {
        let mut xot = Xot::new();
        let doc = xot.parse(r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"><xsl:if _test="false()"></xsl:if></xsl:transform>"#).unwrap();
        preprocess(&mut xot, doc).unwrap();
        assert_eq!(
            xot.to_string(doc).unwrap(),
            r#"<xsl:transform xmlns:xsl="http://www.w3.org/1999/XSL/Transform"><xsl:if test="false()"/></xsl:transform>"#
        );
    }
}
