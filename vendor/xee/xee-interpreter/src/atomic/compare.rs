use std::cmp::Ordering;

use crate::{atomic::Atomic, error};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AtomicCompareValue {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

pub(crate) trait AtomicCompare {
    fn atomic_compare<F>(
        a: Atomic,
        b: Atomic,
        string_compare: F,
        default_offset: chrono::FixedOffset,
    ) -> error::Result<bool>
    where
        F: Fn(&str, &str) -> Ordering;

    // comparison when the arguments are inverted
    // so, a = b is the same as b = a
    // and a < b is the same as b > a
    // and a <= b is the same as b >= a
    fn arguments_inverted() -> impl AtomicCompare;

    // in specialized cases it's nice to have an enum to compare with
    fn value() -> AtomicCompareValue;
}
