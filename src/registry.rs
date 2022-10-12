use crate::person::{patient, Person};
use queue_strategy::{heap_strategy::HeapStrategy, sort_strategy::SortStrategy, QueueI};
use std::cmp::Ordering;
use std::collections::{hash_map::Entry, HashMap};
use std::fmt;

// Medical Registry Number
pub use String as MRN;

pub type WlElement = (MRN, patient::Condition);
pub struct Registry<T = SortStrategy<WlElement>>
where
    T: QueueI<WlElement>,
{
    registry: HashMap<MRN, Person>,
    waiting_list: T,
}

impl Registry<SortStrategy<WlElement>> {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            waiting_list: SortStrategy::new(|&(_, p1_condition), &(_, p2_condition)| -> bool {
                p1_condition < p2_condition
            }),
        }
    }
}

impl Registry<HeapStrategy<WlElement>> {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            waiting_list: HeapStrategy::new(|&(_, p1_condition), &(_, p2_condition)| -> Ordering {
                p1_condition.cmp(&p2_condition)
            }),
        }
    }
}

impl<WaitingList: QueueI<WlElement>> Registry<WaitingList> {
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

impl<WaitingList: QueueI<WlElement>> fmt::Display for Registry<WaitingList> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let delimeter = String::from("\n-----------------\n");
        let head = String::from("- List -");
        let mut result = String::with_capacity(delimeter.capacity() * 2 + head.capacity() * 2);
        for (_, person_enum) in self.registry.iter() {
            result.push_str(&delimeter);
            result.push_str(&format!("{person_enum}"));
            result.push_str(&delimeter);
        }
        // result.push_str("- Waiting List -");
        write!(f, "{result}- Waiting List -\n{:?}", self.waiting_list)
    }
}
