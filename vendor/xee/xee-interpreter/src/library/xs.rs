use xee_xpath_macros::xpath_fn;

use crate::atomic;
use crate::context::DynamicContext;
use crate::error;
use crate::function::StaticFunctionDescription;
use crate::wrap_xpath_fn;

#[xpath_fn("xs:untypedAtomic($arg as xs:anyAtomicType?) as xs:untypedAtomic?")]
fn xs_untyped_atomic(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    Ok(arg.map(|arg| arg.cast_to_untyped_atomic()))
}

#[xpath_fn("xs:numeric($arg as xs:anyAtomicType?) as xs:numeric?")]
fn xs_numeric(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_numeric()).transpose()
}

#[xpath_fn("xs:string($arg as xs:anyAtomicType?) as xs:string?")]
fn xs_string(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    Ok(arg.map(|arg| arg.cast_to_string()))
}

#[xpath_fn("xs:float($arg as xs:anyAtomicType?) as xs:float?")]
fn xs_float(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_float()).transpose()
}

#[xpath_fn("xs:double($arg as xs:anyAtomicType?) as xs:double?")]
fn xs_double(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_double()).transpose()
}

#[xpath_fn("xs:decimal($arg as xs:anyAtomicType?) as xs:decimal?")]
fn xs_decimal(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_decimal()).transpose()
}

#[xpath_fn("xs:integer($arg as xs:anyAtomicType?) as xs:integer?")]
fn xs_integer(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_integer()).transpose()
}

#[xpath_fn("xs:duration($arg as xs:anyAtomicType?) as xs:duration?")]
fn xs_duration(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_duration()).transpose()
}

#[xpath_fn("xs:yearMonthDuration($arg as xs:anyAtomicType?) as xs:yearMonthDuration?")]
fn xs_year_month_duration(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_year_month_duration()).transpose()
}

#[xpath_fn("xs:dayTimeDuration($arg as xs:anyAtomicType?) as xs:dayTimeDuration?")]
fn xs_day_time_duration(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_day_time_duration()).transpose()
}

#[xpath_fn("xs:dateTime($arg as xs:anyAtomicType?) as xs:dateTime?")]
fn xs_date_time(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_date_time()).transpose()
}

#[xpath_fn("xs:dateTimeStamp($arg as xs:anyAtomicType?) as xs:dateTimeStamp?")]
fn xs_date_time_stamp(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_date_time_stamp()).transpose()
}

#[xpath_fn("xs:time($arg as xs:anyAtomicType?) as xs:time?")]
fn xs_time(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_time()).transpose()
}

#[xpath_fn("xs:date($arg as xs:anyAtomicType?) as xs:date?")]
fn xs_date(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_date()).transpose()
}

#[xpath_fn("xs:gYearMonth($arg as xs:anyAtomicType?) as xs:gYearMonth?")]
fn xs_g_year_month(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_g_year_month()).transpose()
}

#[xpath_fn("xs:gYear($arg as xs:anyAtomicType?) as xs:gYear?")]
fn xs_g_year(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_g_year()).transpose()
}

#[xpath_fn("xs:gMonthDay($arg as xs:anyAtomicType?) as xs:gMonthDay?")]
fn xs_g_month_day(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_g_month_day()).transpose()
}

#[xpath_fn("xs:gDay($arg as xs:anyAtomicType?) as xs:gDay?")]
fn xs_g_day(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_g_day()).transpose()
}

#[xpath_fn("xs:gMonth($arg as xs:anyAtomicType?) as xs:gMonth?")]
fn xs_g_month(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_g_month()).transpose()
}

#[xpath_fn("xs:boolean($arg as xs:anyAtomicType?) as xs:boolean?")]
fn xs_boolean(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_boolean()).transpose()
}

#[xpath_fn("xs:base64Binary($arg as xs:anyAtomicType?) as xs:base64Binary?")]
fn xs_base64_binary(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_base64_binary()).transpose()
}

#[xpath_fn("xs:hexBinary($arg as xs:anyAtomicType?) as xs:hexBinary?")]
fn xs_hex_binary(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_hex_binary()).transpose()
}

#[xpath_fn("xs:anyURI($arg as xs:anyAtomicType?) as xs:anyURI?")]
fn xs_any_uri(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_any_uri()).transpose()
}

#[xpath_fn("xs:QName($arg as xs:anyAtomicType?) as xs:QName?")]
fn xs_qname(
    context: &DynamicContext,
    arg: Option<atomic::Atomic>,
) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_qname(context.static_context()))
        .transpose()
}

// string subtypes

#[xpath_fn("xs:normalizedString($arg as xs:anyAtomicType?) as xs:normalizedString?")]
fn xs_normalized_string(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    Ok(arg.map(|arg| arg.cast_to_normalized_string()))
}

#[xpath_fn("xs:token($arg as xs:anyAtomicType?) as xs:token?")]
fn xs_token(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    Ok(arg.map(|arg| arg.cast_to_token()))
}

#[xpath_fn("xs:language($arg as xs:anyAtomicType?) as xs:language?")]
fn xs_language(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_language()).transpose()
}

#[xpath_fn("xs:NMTOKEN($arg as xs:anyAtomicType?) as xs:NMTOKEN?")]
fn xs_nmtoken(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_nmtoken()).transpose()
}

#[xpath_fn("xs:Name($arg as xs:anyAtomicType?) as xs:Name?")]
fn xs_name(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_name()).transpose()
}

#[xpath_fn("xs:NCName($arg as xs:anyAtomicType?) as xs:NCName?")]
fn xs_ncname(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_ncname()).transpose()
}

#[xpath_fn("xs:ID($arg as xs:anyAtomicType?) as xs:ID?")]
fn xs_id(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_id()).transpose()
}

#[xpath_fn("xs:IDREF($arg as xs:anyAtomicType?) as xs:IDREF?")]
fn xs_idref(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_idref()).transpose()
}

#[xpath_fn("xs:ENTITY($arg as xs:anyAtomicType?) as xs:ENTITY?")]
fn xs_entity(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_entity()).transpose()
}

// integer subtypes

#[xpath_fn("xs:long($arg as xs:anyAtomicType?) as xs:long?")]
fn xs_long(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_long()).transpose()
}

#[xpath_fn("xs:int($arg as xs:anyAtomicType?) as xs:int?")]
fn xs_int(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_int()).transpose()
}

#[xpath_fn("xs:short($arg as xs:anyAtomicType?) as xs:short?")]
fn xs_short(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_short()).transpose()
}

#[xpath_fn("xs:byte($arg as xs:anyAtomicType?) as xs:byte?")]
fn xs_byte(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_byte()).transpose()
}

#[xpath_fn("xs:unsignedLong($arg as xs:anyAtomicType?) as xs:unsignedLong?")]
fn xs_unsigned_long(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_unsigned_long()).transpose()
}

#[xpath_fn("xs:unsignedInt($arg as xs:anyAtomicType?) as xs:unsignedInt?")]
fn xs_unsigned_int(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_unsigned_int()).transpose()
}

#[xpath_fn("xs:unsignedShort($arg as xs:anyAtomicType?) as xs:unsignedShort?")]
fn xs_unsigned_short(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_unsigned_short()).transpose()
}

#[xpath_fn("xs:unsignedByte($arg as xs:anyAtomicType?) as xs:unsignedByte?")]
fn xs_unsigned_byte(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_unsigned_byte()).transpose()
}

#[xpath_fn("xs:nonPositiveInteger($arg as xs:anyAtomicType?) as xs:nonPositiveInteger?")]
fn xs_non_positive_integer(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_non_positive_integer())
        .transpose()
}

#[xpath_fn("xs:negativeInteger($arg as xs:anyAtomicType?) as xs:negativeInteger?")]
fn xs_negative_integer(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_negative_integer()).transpose()
}

#[xpath_fn("xs:nonNegativeInteger($arg as xs:anyAtomicType?) as xs:nonNegativeInteger?")]
fn xs_non_negative_integer(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_non_negative_integer())
        .transpose()
}

#[xpath_fn("xs:positiveInteger($arg as xs:anyAtomicType?) as xs:positiveInteger?")]
fn xs_positive_integer(arg: Option<atomic::Atomic>) -> error::Result<Option<atomic::Atomic>> {
    arg.map(|arg| arg.cast_to_positive_integer()).transpose()
}

pub(crate) fn static_function_descriptions() -> Vec<StaticFunctionDescription> {
    vec![
        wrap_xpath_fn!(xs_untyped_atomic),
        wrap_xpath_fn!(xs_numeric),
        wrap_xpath_fn!(xs_string),
        wrap_xpath_fn!(xs_float),
        wrap_xpath_fn!(xs_double),
        wrap_xpath_fn!(xs_decimal),
        wrap_xpath_fn!(xs_integer),
        wrap_xpath_fn!(xs_duration),
        wrap_xpath_fn!(xs_year_month_duration),
        wrap_xpath_fn!(xs_day_time_duration),
        wrap_xpath_fn!(xs_date_time),
        wrap_xpath_fn!(xs_date_time_stamp),
        wrap_xpath_fn!(xs_time),
        wrap_xpath_fn!(xs_date),
        wrap_xpath_fn!(xs_g_year_month),
        wrap_xpath_fn!(xs_g_year),
        wrap_xpath_fn!(xs_g_month_day),
        wrap_xpath_fn!(xs_g_day),
        wrap_xpath_fn!(xs_g_month),
        wrap_xpath_fn!(xs_boolean),
        wrap_xpath_fn!(xs_base64_binary),
        wrap_xpath_fn!(xs_hex_binary),
        wrap_xpath_fn!(xs_any_uri),
        wrap_xpath_fn!(xs_qname),
        // string subtypes
        wrap_xpath_fn!(xs_normalized_string),
        wrap_xpath_fn!(xs_token),
        wrap_xpath_fn!(xs_language),
        wrap_xpath_fn!(xs_nmtoken),
        wrap_xpath_fn!(xs_name),
        wrap_xpath_fn!(xs_ncname),
        wrap_xpath_fn!(xs_id),
        wrap_xpath_fn!(xs_idref),
        wrap_xpath_fn!(xs_entity),
        // integer subtypes
        wrap_xpath_fn!(xs_long),
        wrap_xpath_fn!(xs_int),
        wrap_xpath_fn!(xs_short),
        wrap_xpath_fn!(xs_byte),
        wrap_xpath_fn!(xs_unsigned_long),
        wrap_xpath_fn!(xs_unsigned_int),
        wrap_xpath_fn!(xs_unsigned_short),
        wrap_xpath_fn!(xs_unsigned_byte),
        wrap_xpath_fn!(xs_non_positive_integer),
        wrap_xpath_fn!(xs_negative_integer),
        wrap_xpath_fn!(xs_non_negative_integer),
        wrap_xpath_fn!(xs_positive_integer),
    ]
}
