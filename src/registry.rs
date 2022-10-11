mod waiting_list;
use crate::person::Person;
use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};
use std::fmt;
use waiting_list::QueueStrategy;

// Medical Registry Number
pub use String as MRN;

pub struct Registry<WaitingList = waiting_list::SortStrategy<Person>> {
    registry: HashMap<MRN, Person>,
    waiting_list: WaitingList,
}

pub fn new() -> Registry {
    Registry {
        registry: HashMap::new(),
        waiting_list: QueueStrategy::new(|p1: &Person, p2: &Person| -> bool {
            p1.get_age() < p2.get_age()
        }),
    }
}

impl Registry {
    pub fn add(&mut self, person: Person) -> Result<&Person, String> {
        match self.registry.entry(person.get_mrn().clone()) {
            Vacant(v) => Ok(v.insert(person)),
            Occupied(o) => Err(format!(
                "There is already a person registered with that identifier, {}",
                o.key(),
            )),
        }
    }
    pub fn remove(&mut self, mrn: &MRN) -> Result<Person, String> {
        match self.registry.remove(mrn) {
            Some(p) => Ok(p),
            None => Err(format!("No one is registered with MRN {mrn}")),
        }
    }
    pub fn find(&self, mrn: &MRN) -> Option<&Person> {
        self.registry.get(mrn)
    }
    pub fn find_by_name(&self, name: &String) -> Vec<&Person> {
        self.registry
            .iter()
            .filter(|(_, person)| person.get_name() != name)
            .map(|(_, person)| person)
            .collect()
    }
}

impl fmt::Display for Registry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let delimeter = String::from("\n-----------------\n");
        let mut result = String::with_capacity(delimeter.capacity() * 2);
        for (_, person_enum) in self.registry.iter() {
            result.push_str(&delimeter);
            result.push_str(&format!("{person_enum}"));
            result.push_str(&delimeter);
        }
        write!(f, "{result}")
    }
}
