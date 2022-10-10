pub mod doctor;
pub mod patient;

use crate::registry::MRN;
use crate::utils::num_generator::NumGenerator;

#[derive(Copy, Clone)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Copy, Clone)]
pub enum Condition {
    Employee,
    GoodCondition,
    SlightIllness,
    SeriousIllness,
    CriticalCondition,
}

pub enum PersonE {
    Patient(Person<patient::Patient>),
    Doctor(Person<doctor::Doctor>),
}

impl PersonE {
    pub fn get_name(&self) -> &String {
        match self {
            PersonE::Patient(p) => &p.name,
            PersonE::Doctor(d) => &d.name,
        }
    }
    pub fn get_age(&self) -> i32 {
        match self {
            PersonE::Patient(p) => p.age,
            PersonE::Doctor(d) => d.age,
        }
    }
    pub fn get_gender(&self) -> Gender {
        match self {
            PersonE::Patient(p) => p.gender,
            PersonE::Doctor(d) => d.gender,
        }
    }
    pub fn get_status(&self) -> Condition {
        match self {
            PersonE::Patient(p) => p.condition,
            PersonE::Doctor(d) => d.condition,
        }
    }
    pub fn get_mrn(&self) -> &MRN {
        match self {
            PersonE::Patient(p) => &p.mrn,
            PersonE::Doctor(d) => &d.mrn,
        }
    }
}

use once_cell::sync::Lazy;
use std::sync::Mutex;
static NUM_GEN: Lazy<Mutex<NumGenerator>> = Lazy::new(|| Mutex::new(NumGenerator::default()));

pub struct Person<T> {
    name: String,
    age: i32,
    gender: Gender,
    condition: Condition,
    kind: T,

    mrn: MRN,
}

impl<T> Person<T> {
    fn new(
        name: String,
        age: i32,
        gender: Gender,
        condition: Condition,
        kind: T,
    ) -> Result<Self, String> {
        Ok(Person {
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

impl<T> Drop for Person<T> {
    fn drop(&mut self) {
        println!("Calling drop with {}", self.name);
    }
}
