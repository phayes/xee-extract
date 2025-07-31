// https://www.w3.org/TR/xpath-functions-31/#durations

use ibig::IBig;
use rust_decimal::Decimal;
use xee_xpath_macros::xpath_fn;

use crate::atomic::Duration;
use crate::function::StaticFunctionDescription;
use crate::wrap_xpath_fn;

#[xpath_fn("fn:years-from-duration($arg as xs:duration?) as xs:integer?")]
fn years_from_duration(arg: Option<Duration>) -> Option<IBig> {
    if let Some(arg) = arg {
        Some(arg.year_month.years().into())
    } else {
        None
    }
}

#[xpath_fn("fn:months-from-duration($arg as xs:duration?) as xs:integer?")]
fn months_from_duration(arg: Option<Duration>) -> Option<IBig> {
    if let Some(arg) = arg {
        Some(arg.year_month.months().into())
    } else {
        None
    }
}

#[xpath_fn("fn:days-from-duration($arg as xs:duration?) as xs:integer?")]
fn days_from_duration(arg: Option<Duration>) -> Option<IBig> {
    if let Some(arg) = arg {
        Some(arg.day_time.num_days().into())
    } else {
        None
    }
}

#[xpath_fn("fn:hours-from-duration($arg as xs:duration?) as xs:integer?")]
fn hours_from_duration(arg: Option<Duration>) -> Option<IBig> {
    if let Some(arg) = arg {
        let seconds = arg.day_time.num_seconds();
        Some(((seconds % 86400) / 3600).into())
    } else {
        None
    }
}

#[xpath_fn("fn:minutes-from-duration($arg as xs:duration?) as xs:integer?")]
fn minutes_from_duration(arg: Option<Duration>) -> Option<IBig> {
    if let Some(arg) = arg {
        let seconds = arg.day_time.num_seconds();
        Some(((seconds % 3600) / 60).into())
    } else {
        None
    }
}

#[xpath_fn("fn:seconds-from-duration($arg as xs:duration?) as xs:decimal?")]
fn seconds_from_duration(arg: Option<Duration>) -> Option<Decimal> {
    if let Some(arg) = arg {
        let ss = arg.day_time.num_milliseconds();
        let ss: Decimal = ss.into();
        let ss = (ss / Decimal::from(1000)) % Decimal::from(60);
        Some(ss)
    } else {
        None
    }
}

pub(crate) fn static_function_descriptions() -> Vec<StaticFunctionDescription> {
    vec![
        wrap_xpath_fn!(years_from_duration),
        wrap_xpath_fn!(months_from_duration),
        wrap_xpath_fn!(days_from_duration),
        wrap_xpath_fn!(hours_from_duration),
        wrap_xpath_fn!(minutes_from_duration),
        wrap_xpath_fn!(seconds_from_duration),
    ]
}
