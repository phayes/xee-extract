//! Custom atomic types for XPath.
//!
//! Where atomic types cannot be defined using standard Rust types or
//! external packages such as [`chrono`], [`ordered_float`] and [`rust_decimal`],
//! Xee defines its own types.

pub use xee_interpreter::atomic::{
    BinaryType, Duration, GDay, GMonth, GMonthDay, GYear, GYearMonth, NaiveDateTimeWithOffset,
    NaiveDateWithOffset, NaiveTimeWithOffset, StringType, YearMonthDuration,
};
