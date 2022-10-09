pub mod doctor;
pub mod patient;

use crate::num_Generator::NumGenerator;
use crate::registry::MRN;

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

// static p: std::sync::Mutex<i32> = std::sync::Mutex::new(3);
pub struct PersonS<T> {
    name: String,
    age: i32,
    gender: Gender,
    condition: Condition,
    kind: T,

    mrn: MRN,
}

fn new<T>(
    name: String,
    age: i32,
    gender: Gender,
    condition: Condition,
    kind: T,
    num_generator: &mut NumGenerator,
) -> PersonS<T> {
    PersonS {
        name,
        age,
        gender,
        condition,
        kind,
        mrn: num_generator
            .next()
            .expect("The system run out of posible identifiers"),
    }
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
    pub fn get_status(&self) -> Condition {
        match self {
            Person::Patient(p) => p.condition,
            Person::Doctor(d) => d.condition,
        }
    }
}
