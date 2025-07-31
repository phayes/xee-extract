use crate::{function, pattern::ModeLookup};

#[derive(Debug)]
pub struct Declarations {
    pub mode_lookup: ModeLookup<function::InlineFunctionId>,
}

impl Declarations {
    pub(crate) fn new() -> Self {
        Self {
            mode_lookup: ModeLookup::new(),
        }
    }
}
