use std::collections::HashMap;

// Medical Registry Number
use String as MRN;

pub struct Registry {
    registy: HashMap<MRN, Box<dyn crate::PersonTrait>>,
}
