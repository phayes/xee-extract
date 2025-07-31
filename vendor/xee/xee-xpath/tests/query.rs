use ibig::{ibig, IBig};
use xee_interpreter::sequence::Sequence;
use xee_xpath::{error, query::RecurseQuery, Documents, Item, Queries, Query, Recurse};

#[test]
fn test_duplicate_document_uri() -> error::Result<()> {
    let mut documents = Documents::new();
    let _doc1 = documents
        .add_string(
            "http://example.com/doc1".try_into().unwrap(),
            r#"<doc><result><any-of><value>A</value></any-of></result></doc>"#,
        )
        .unwrap();
    // try to load doc with the same URI
    let doc2_err = documents
        .add_string(
            "http://example.com/doc1".try_into().unwrap(),
            r#"<doc><result><value>A</value></result></doc>"#,
        )
        .unwrap_err();
    assert_eq!(
        doc2_err.to_string(),
        "Duplicate URI: http://example.com/doc1"
    );
    Ok(())
}

#[test]
fn test_simple_query() -> error::Result<()> {
    let mut documents = Documents::new();
    let doc = documents
        .add_string("http://example.com".try_into().unwrap(), "<root>foo</root>")
        .unwrap();

    let queries = Queries::default();
    let q = queries.one("/root/string()", |_, item| {
        Ok(item.try_into_value::<String>()?)
    })?;

    let r = q.execute(&mut documents, doc)?;
    assert_eq!(r, "foo");
    Ok(())
}

#[test]
fn test_sequence_query() -> error::Result<()> {
    let mut documents = Documents::new();
    let doc = documents
        .add_string("http://example.com".try_into().unwrap(), "<root>foo</root>")
        .unwrap();

    let queries = Queries::default();
    let q = queries.sequence("/root/string()")?;

    let r = q.execute(&mut documents, doc)?;
    let sequence: Sequence = "foo".into();
    assert_eq!(r, sequence);
    Ok(())
}

#[test]
fn test_option_query() -> error::Result<()> {
    let mut documents = Documents::new();
    let doc_with_value = documents
        .add_string(
            "http://example.com/with_value".try_into().unwrap(),
            "<root><value>Foo</value></root>",
        )
        .unwrap();
    let doc_without_value = documents
        .add_string(
            "http://example.com/without_value".try_into().unwrap(),
            "<root></root>",
        )
        .unwrap();

    let queries = Queries::default();
    let q = queries.option("/root/value/string()", |_, item| {
        Ok(item.try_into_value::<String>()?)
    })?;

    let r = q.execute(&mut documents, doc_with_value)?;
    assert_eq!(r, Some("Foo".to_string()));
    let r = q.execute(&mut documents, doc_without_value)?;
    assert_eq!(r, None);
    Ok(())
}

#[test]
fn test_nested_query() -> error::Result<()> {
    let mut documents = Documents::new();
    let doc = documents
        .add_string(
            "http://example.com".try_into().unwrap(),
            "<root><a>1</a><a>2</a></root>",
        )
        .unwrap();

    let queries = Queries::default();

    let f_query = queries.one("./number()", |_, item| Ok(item.try_into_value::<f64>()?))?;
    let q = queries.many("/root/a", |context, item| f_query.execute(context, item))?;

    let r = q.execute(&mut documents, doc)?;
    assert_eq!(r, vec![1.0, 2.0]);
    Ok(())
}

#[test]
fn test_option_query_recurse() -> error::Result<()> {
    let queries = Queries::default();

    #[derive(Debug, PartialEq, Eq)]
    enum Expr {
        AnyOf(Box<Expr>),
        Value(String),
        Empty,
    }

    // if we find the "any-of" element, we want to use a recursive
    // call to the query we pass it
    let any_of_recurse = queries.option_recurse("any-of")?;
    // the "value" element is simply a string
    let value_query = queries.option("value/string()", |_, item| {
        Ok(item.try_into_value::<String>()?)
    })?;

    // a result is either a "value" or an "any-of" element
    let result_query = queries.one("/doc/result", |documents, item| {
        let f = |documents: &mut Documents, item: &Item, recurse: &Recurse<_>| {
            // we either call the any of query, which recursively
            // calls this function
            if let Some(any_of) = any_of_recurse.execute(documents, item, recurse)? {
                return Ok(Expr::AnyOf(Box::new(any_of)));
            }
            // or use the value query
            if let Some(value) = value_query.execute(documents, item)? {
                return Ok(Expr::Value(value));
            }
            Ok(Expr::Empty)
        };
        // we want to recursively call this function
        let recurse = Recurse::new(&f);
        recurse.execute(documents, item)
    })?;

    let mut documents = Documents::new();
    let doc1 = documents
        .add_string(
            "http://example.com/doc1".try_into().unwrap(),
            r#"<doc><result><any-of><value>A</value></any-of></result></doc>"#,
        )
        .unwrap();
    let doc2 = documents
        .add_string(
            "http://example.com/doc2".try_into().unwrap(),
            r#"<doc><result><value>A</value></result></doc>"#,
        )
        .unwrap();

    let r = result_query.execute(&mut documents, doc1)?;
    assert_eq!(r, Expr::AnyOf(Box::new(Expr::Value("A".to_string()))));

    let r = result_query.execute(&mut documents, doc2)?;
    assert_eq!(r, Expr::Value("A".to_string()));
    Ok(())
}

#[test]
fn test_many_query_recurse() -> error::Result<()> {
    let queries = Queries::default();

    #[derive(Debug, PartialEq, Eq)]
    enum Expr {
        AnyOf(Vec<Expr>),
        Value(String),
        Empty,
    }

    // if we find any "any-of" element, we want to use a recursive
    // call to the query we pass it
    let any_of_recurse = queries.many_recurse("any-of")?;
    // the "value" element is simply a string
    let value_query = queries.option("value/string()", |_, item| {
        Ok(item.try_into_value::<String>()?)
    })?;

    // a result is either a "value" or an "any-of" element
    let result_query = queries.one("/doc/result", |documents, item| {
        let f = |documents: &mut Documents, item: &Item, recurse: &Recurse<_>| {
            // we either call the any of query, which recursively
            // calls this function
            let elements = any_of_recurse.execute(documents, item, recurse)?;
            if !elements.is_empty() {
                return Ok(Expr::AnyOf(elements));
            }
            // or use the value query
            if let Some(value) = value_query.execute(documents, item)? {
                return Ok(Expr::Value(value));
            }
            Ok(Expr::Empty)
        };
        // we want to recursively call this function
        let recurse = Recurse::new(&f);
        recurse.execute(documents, item)
    })?;

    let mut documents = Documents::new();
    let doc1 = documents
        .add_string(
            "http://example.com/doc1".try_into().unwrap(),
            r#"<doc><result><any-of><value>A</value></any-of><any-of><value>B</value></any-of></result></doc>"#,
        )
        .unwrap();
    let doc2 = documents
        .add_string(
            "http://example.com/doc2".try_into().unwrap(),
            r#"<doc><result><value>A</value></result></doc>"#,
        )
        .unwrap();

    let r = result_query.execute(&mut documents, doc1)?;
    assert_eq!(
        r,
        Expr::AnyOf(vec![
            Expr::Value("A".to_string()),
            Expr::Value("B".to_string())
        ])
    );

    let r = result_query.execute(&mut documents, doc2)?;
    assert_eq!(r, Expr::Value("A".to_string()));
    Ok(())
}

#[test]
fn test_map_query() -> error::Result<()> {
    let queries = Queries::default();
    let q = queries
        .one("1 + 2", |_, item| {
            let v: IBig = item.to_atomic()?.try_into()?;
            Ok(v)
        })?
        .map(|v, _, _| Ok(v + ibig!(1)));

    let mut documents = Documents::new();

    let r = q.execute(&mut documents, &1i64.into())?;
    assert_eq!(r, ibig!(4));
    Ok(())
}

#[test]
fn test_map_query_clone() -> error::Result<()> {
    let queries = Queries::default();
    let q = queries
        .one("1 + 2", |_, item| {
            let v: IBig = item.to_atomic()?.try_into()?;
            Ok(v)
        })?
        .map(|v, _, _| Ok(v + ibig!(1)));
    let q = q.clone();
    let mut documents = Documents::new();

    let r = q.execute(&mut documents, &1i64.into())?;
    assert_eq!(r, ibig!(4));
    Ok(())
}
