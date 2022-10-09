use std::collections::HashMap;
use crate::Person;

// Medical Registry Number
pub use String as MRN;

pub struct Registry {
    registy: HashMap<MRN, Person>,
}
