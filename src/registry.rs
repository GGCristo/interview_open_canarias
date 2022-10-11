mod waiting_list;
use crate::person::{patient, Person};
use std::collections::{hash_map::Entry, HashMap};
use std::fmt;
use waiting_list::{QueueI, SortStrategy};

// Medical Registry Number
pub use String as MRN;

pub struct Registry<WaitingList = SortStrategy<(MRN, patient::Condition)>> {
    registry: HashMap<MRN, Person>,
    waiting_list: WaitingList,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            waiting_list: QueueI::new(|&(_, p1_condition), &(_, p2_condition)| -> bool {
                p1_condition < p2_condition
            }),
        }
    }
    pub fn add(&mut self, person: Person) -> Result<&Person, String> {
        match self.registry.entry(person.get_mrn().clone()) {
            Entry::Vacant(v) => {
                let person = v.insert(person);
                if let Person::Patient(patient) = &person {
                    self.waiting_list
                        .insert((person.get_mrn().to_string(), patient.get_condition()));
                }
                Ok(person)
            }
            Entry::Occupied(o) => Err(format!(
                "There is already a person registered with that identifier, {}",
                o.key(),
            )),
        }
    }
    pub fn remove(&mut self, mrn: &MRN) -> Result<Person, String> {
        let _ = self.waiting_list.remove_one_by(|(i_mrn, _)| i_mrn == mrn);
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
            .values()
            .filter(|person| person.get_name() != name)
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
