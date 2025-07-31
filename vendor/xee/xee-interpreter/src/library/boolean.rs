// https://www.w3.org/TR/xpath-functions-31/#boolean-functions
use xee_xpath_macros::xpath_fn;

use crate::error;
use crate::function::StaticFunctionDescription;
use crate::sequence;
use crate::wrap_xpath_fn;

#[xpath_fn("fn:true() as xs:boolean")]
fn true_() -> bool {
    true
}

#[xpath_fn("fn:false() as xs:boolean")]
fn false_() -> bool {
    false
}

#[xpath_fn("fn:not($arg as item()*) as xs:boolean")]
fn not(arg: &sequence::Sequence) -> error::Result<bool> {
    arg.effective_boolean_value().map(|b| !b)
}

#[xpath_fn("fn:boolean($arg as item()*) as xs:boolean")]
fn boolean(arg: &sequence::Sequence) -> error::Result<bool> {
    arg.effective_boolean_value()
}

pub(crate) fn static_function_descriptions() -> Vec<StaticFunctionDescription> {
    vec![
        wrap_xpath_fn!(true_),
        wrap_xpath_fn!(false_),
        wrap_xpath_fn!(not),
        wrap_xpath_fn!(boolean),
    ]
}
