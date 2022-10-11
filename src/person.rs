pub mod doctor;
pub mod patient;

use crate::registry::MRN;
use crate::utils::num_generator::NumGenerator;
use once_cell::sync::Lazy; // Until LazyLock is stable https://doc.rust-lang.org/std/sync/struct.LazyLock.html
use std::fmt;
use std::sync::Mutex;

#[derive(Copy, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Copy, Clone, Debug)]
pub enum Condition {
    Employee,
    GoodCondition,
    SlightIllness,
    SeriousIllness,
    CriticalCondition,
}

pub enum Person {
    Patient(PersonS<patient::Patient>),
    Doctor(PersonS<doctor::Doctor>),
}

impl Person {
    pub fn get_name(&self) -> &String {
        match self {
            Person::Patient(p) => &p.name,
            Person::Doctor(d) => &d.name,
        }
    }
    pub fn get_age(&self) -> i32 {
        match self {
            Person::Patient(p) => p.age,
            Person::Doctor(d) => d.age,
        }
    }
    pub fn get_gender(&self) -> Gender {
        match self {
            Person::Patient(p) => p.gender,
            Person::Doctor(d) => d.gender,
        }
    }
    pub fn get_condition(&self) -> Condition {
        match self {
            Person::Patient(p) => p.condition,
            Person::Doctor(d) => d.condition,
        }
    }
    pub fn get_mrn(&self) -> &MRN {
        match self {
            Person::Patient(p) => &p.mrn,
            Person::Doctor(d) => &d.mrn,
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!(
            "MRN: {}\nName: {}\nAge: {}\nGender: {:?}\nCondition: {:?}\n",
            self.get_mrn(),
            self.get_name(),
            self.get_age(),
            self.get_gender(),
            self.get_condition()
        );
        match self {
            Person::Doctor(d) => {
                result.push_str(&format!("Specialty: {:?}\n", d.get_specialty()));
            }
            Person::Patient(p) => {
                result.push_str(&format!("Notes: {:?}\n", p.get_notes()));
            }
        }
        write!(f, "{result}")
    }
}

static NUM_GEN: Lazy<Mutex<NumGenerator>> = Lazy::new(|| Mutex::new(NumGenerator::default()));

pub struct PersonS<T> {
    name: String,
    age: i32,
    gender: Gender,
    condition: Condition,
    kind: T,

    mrn: MRN,
}

impl<T> PersonS<T> {
    fn new(
        name: String,
        age: i32,
        gender: Gender,
        condition: Condition,
        kind: T,
    ) -> Result<Self, String> {
        Ok(PersonS {
            name,
            age,
            gender,
            condition,
            kind,
            mrn: NUM_GEN
                .lock()
                .unwrap()
                .next()
                .ok_or("The system run out of posible identifiers")?,
        })
    }
}

impl<T> Drop for PersonS<T> {
    fn drop(&mut self) {
        NUM_GEN.lock().unwrap().free(self.mrn.clone());
    }
}
