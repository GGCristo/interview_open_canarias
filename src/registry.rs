use crate::person::PersonE;
use std::collections::{hash_map, HashMap};
use std::fmt;

// Medical Registry Number
pub use String as MRN;

pub struct Registry {
    registy: HashMap<MRN, PersonE>,
}

pub fn new() -> Registry {
    Registry {
        registy: HashMap::new(),
    }
}

impl Registry {
    pub fn add(&mut self, person: PersonE) -> Result<&PersonE, String> {
        match self.registy.entry(person.get_mrn().clone()) {
            hash_map::Entry::Vacant(v) => Ok(v.insert(person)),
            hash_map::Entry::Occupied(o) => Err(format!(
                "There is already a person registered with that identifier, {}",
                o.key(),
            )),
        }
    }
}

// impl fmt::Display for Registry {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, )
//     }
// }
