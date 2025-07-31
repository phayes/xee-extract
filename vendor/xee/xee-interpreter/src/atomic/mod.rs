/// Atomic values.
///
/// XPath defines a host of Atomic values, and rules for how to do arithmetic
/// on them, compare them, and cast them to other value types.
mod atomic_core;
mod cast;
mod cast_binary;
mod cast_datetime;
mod cast_numeric;
mod cast_string;
mod compare;
mod datetime;
mod map_key;
mod op_add;
mod op_div;
mod op_eq;
mod op_ge;
mod op_gt;
mod op_idiv;
mod op_le;
mod op_lt;
mod op_mod;
mod op_multiply;
mod op_ne;
mod op_subtract;
mod op_unary;
mod round;
mod types;
mod xpath_repr;

pub use atomic_core::Atomic;
pub(crate) use compare::{AtomicCompare, AtomicCompareValue};
pub(crate) use datetime::ToDateTimeStamp;
pub use datetime::{
    Duration, GDay, GMonth, GMonthDay, GYear, GYearMonth, NaiveDateTimeWithOffset,
    NaiveDateWithOffset, NaiveTimeWithOffset, YearMonthDuration,
};
pub(crate) use map_key::MapKey;
pub(crate) use op_add::op_add;
pub(crate) use op_div::op_div;
pub(crate) use op_eq::OpEq;
pub(crate) use op_ge::OpGe;
pub(crate) use op_gt::OpGt;
pub(crate) use op_idiv::op_idiv;
pub(crate) use op_le::OpLe;
pub(crate) use op_lt::OpLt;
pub(crate) use op_mod::op_mod;
pub(crate) use op_multiply::op_multiply;
pub(crate) use op_ne::OpNe;
pub(crate) use op_subtract::op_subtract;
pub(crate) use round::{round_atomic, round_half_to_even_atomic};
pub use types::{BinaryType, IntegerType, StringType};
