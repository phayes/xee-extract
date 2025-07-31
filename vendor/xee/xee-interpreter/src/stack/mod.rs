/// Values on the the interpreter stack. A value is either empty,
/// a single item, or a sequence of items, or a special marker absent.
/// The sequence module wraps around this to create a sequence API.
mod comparison;
mod value;

pub use value::Value;
