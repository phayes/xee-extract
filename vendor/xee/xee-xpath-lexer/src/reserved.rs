use crate::Token;

impl<'a> Token<'a> {
    // tokens that can count as an ncname as a local name or as a prefix
    pub(crate) fn ncname(&self) -> Option<&'a str> {
        // in section A.3 of the XPath 3.1 specification
        // a bunch of tokens are listed as reserved functions.
        // They can be used as a valid prefix or local name, just like
        // an ncname
        match self {
            Token::Array => Some("array"),
            Token::Attribute => Some("attribute"),
            Token::Comment => Some("comment"),
            Token::DocumentNode => Some("document-node"),
            Token::Element => Some("element"),
            Token::EmptySequence => Some("empty-sequence"),
            Token::Function => Some("function"),
            Token::If => Some("if"),
            Token::Item => Some("item"),
            Token::Map => Some("map"),
            Token::NamespaceNode => Some("namespace-node"),
            Token::Node => Some("node"),
            Token::ProcessingInstruction => Some("processing-instruction"),
            Token::SchemaAttribute => Some("schema-attribute"),
            Token::SchemaElement => Some("schema-element"),
            Token::Switch => Some("switch"),
            Token::Text => Some("text"),
            Token::Typeswitch => Some("typeswitch"),

            // an NCName of course can also be a prefix or a local name
            Token::NCName(name) => Some(name),
            _ => None,
        }
    }
}
