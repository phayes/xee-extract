use std::rc::Rc;

use xee_interpreter::{
    context::{self, StaticContext},
    error::SpannedResult as Result,
};
use xee_xpath_compiler::parse;

use crate::query::{
    Convert, ManyQuery, ManyRecurseQuery, OneQuery, OneRecurseQuery, OptionQuery,
    OptionRecurseQuery, SequenceQuery,
};

/// A collection of XPath queries
///
/// You can register xpath expressions with conversion functions
/// to turn the results into Rust values.
#[derive(Debug, Default)]
pub struct Queries<'a> {
    pub(crate) default_static_context_builder: context::StaticContextBuilder<'a>,
}

impl<'a> Queries<'a> {
    /// Construct a new collection of queries
    ///
    /// Supply a default static context builder, which is used
    /// by default to construct a static context if none is supplied
    /// explicitly.
    pub fn new(default_static_context_builder: context::StaticContextBuilder<'a>) -> Self {
        Self {
            default_static_context_builder,
        }
    }

    /// Construct a query that expects a single item result.
    ///
    /// This item is converted into a Rust value using supplied `convert` function.
    ///
    /// This uses a default static context.
    pub fn one<V, F>(&self, s: &str, convert: F) -> Result<OneQuery<V, F>>
    where
        F: Convert<V>,
    {
        self.one_with_context(s, convert, self.default_static_context_builder.build())
    }

    /// Construct a query that expects a single item result.
    ///
    /// This item is converted into a Rust value using supplied `convert` function.
    ///
    /// You can supply a static context explicitly.
    pub fn one_with_context<V, F>(
        &self,
        s: &str,
        convert: F,
        static_context: StaticContext,
    ) -> Result<OneQuery<V, F>>
    where
        F: Convert<V>,
    {
        Ok(OneQuery {
            program: Rc::new(parse(static_context, s)?),
            convert,
            phantom: std::marker::PhantomData,
        })
    }

    /// Construct a query that expects a single item result.
    ///
    /// This item is converted into a Rust value not using a convert function
    /// but through a recursive call that's passed in during execution.
    ///
    /// NOTE: recursion generally needs a stopping condition, but `one_recurse`
    /// expects one value always - unlike `option_recurse` and `many_recurse`
    /// which have the None or empty value. I think this means that
    /// `one_recurse` is not in fact useful.
    pub fn one_recurse(&self, s: &str) -> Result<OneRecurseQuery> {
        self.one_recurse_with_context(s, self.default_static_context_builder.build())
    }

    /// Construct a query that expects a single item result, with explicit
    /// static context.
    pub fn one_recurse_with_context(
        &self,
        s: &str,
        static_context: context::StaticContext,
    ) -> Result<OneRecurseQuery> {
        Ok(OneRecurseQuery {
            program: Rc::new(parse(static_context, s)?),
        })
    }

    /// Construct a query that expects an optional single item result.
    ///
    /// This item is converted into a Rust value using supplied `convert` function.
    pub fn option<V, F>(&self, s: &str, convert: F) -> Result<OptionQuery<V, F>>
    where
        F: Convert<V>,
    {
        self.option_with_context(s, convert, self.default_static_context_builder.build())
    }

    /// Construct a query that expects an optional single item result with
    /// explicit static context.
    pub fn option_with_context<V, F>(
        &self,
        s: &str,
        convert: F,
        static_context: context::StaticContext,
    ) -> Result<OptionQuery<V, F>>
    where
        F: Convert<V>,
    {
        Ok(OptionQuery {
            program: Rc::new(parse(static_context, s)?),
            convert,
            phantom: std::marker::PhantomData,
        })
    }

    /// Construct a recursive query that expects an optional single item result.
    ///
    /// This item is converted into a Rust value not using a convert
    /// function but through a recursive call that's passed in during
    /// execution.
    pub fn option_recurse(&self, s: &str) -> Result<OptionRecurseQuery> {
        self.option_recurse_with_context(s, self.default_static_context_builder.build())
    }

    /// Construct a recursive query that expects an optional single item result, with
    /// explicit static context.
    pub fn option_recurse_with_context(
        &self,
        s: &str,
        static_context: context::StaticContext,
    ) -> Result<OptionRecurseQuery> {
        Ok(OptionRecurseQuery {
            program: Rc::new(parse(static_context, s)?),
        })
    }

    /// Construct a query that expects many items as a result.
    ///
    /// These items are converted into Rust values using supplied `convert` function.
    pub fn many<V, F>(&self, s: &str, convert: F) -> Result<ManyQuery<V, F>>
    where
        F: Convert<V>,
    {
        self.many_with_context(s, convert, self.default_static_context_builder.build())
    }

    /// Construct a query that expects many items as a result, with explicit
    /// static context.
    pub fn many_with_context<V, F>(
        &self,
        s: &str,
        convert: F,
        static_context: context::StaticContext,
    ) -> Result<ManyQuery<V, F>>
    where
        F: Convert<V>,
    {
        Ok(ManyQuery {
            program: Rc::new(parse(static_context, s)?),
            convert,
            phantom: std::marker::PhantomData,
        })
    }

    /// Construct a query that expects many items as a result.
    ///
    /// These items are converted into Rust values not using a convert
    /// function but through a recursive call that's passed in during
    /// execution.
    pub fn many_recurse(&self, s: &str) -> Result<ManyRecurseQuery> {
        self.many_recurse_with_context(s, self.default_static_context_builder.build())
    }

    /// Construct a recursive query that expects many items as a result, with explicit
    /// static context.
    pub fn many_recurse_with_context(
        &self,
        s: &str,
        static_context: context::StaticContext,
    ) -> Result<ManyRecurseQuery> {
        Ok(ManyRecurseQuery {
            program: Rc::new(parse(static_context, s)?),
        })
    }

    /// Construct a query that gets a [`Sequence`] as a result.
    ///
    /// This is a low-level API that allows you to get the raw sequence
    /// without converting it into Rust values.
    pub fn sequence(&self, s: &str) -> Result<SequenceQuery> {
        self.sequence_with_context(s, self.default_static_context_builder.build())
    }

    /// Construct a query that gets a [`Sequence`] as a result, with explicit
    /// static context.
    pub fn sequence_with_context(
        &self,
        s: &str,
        static_context: context::StaticContext,
    ) -> Result<SequenceQuery> {
        Ok(SequenceQuery {
            program: Rc::new(parse(static_context, s)?),
        })
    }
}

#[cfg(test)]
mod tests {

    use iri_string::types::IriStr;

    use crate::{query::Query, Documents};

    use super::*;

    #[test]
    fn test_one_query() -> Result<()> {
        let mut documents = Documents::new();
        let uri: &IriStr = "http://example.com".try_into().unwrap();
        let doc = documents.add_string(uri, "<root>foo</root>").unwrap();

        let queries = Queries::default();
        let q = queries.one("/root/string()", |_, item| {
            Ok(item.try_into_value::<String>()?)
        })?;

        let r = q.execute(&mut documents, doc)?;
        assert_eq!(r, "foo");
        Ok(())
    }
}
