pub mod doctor;
pub mod patient;

use crate::registry::MRN;
use crate::utils::num_generator::NumGenerator;
use once_cell::sync::Lazy;
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

pub enum PersonEnum {
    Patient(Person<patient::Patient>),
    Doctor(Person<doctor::Doctor>),
}

impl PersonEnum {
    pub fn get_name(&self) -> &String {
        match self {
            PersonEnum::Patient(p) => &p.name,
            PersonEnum::Doctor(d) => &d.name,
        }
    }
    pub fn get_age(&self) -> i32 {
        match self {
            PersonEnum::Patient(p) => p.age,
            PersonEnum::Doctor(d) => d.age,
        }
    }
    pub fn get_gender(&self) -> Gender {
        match self {
            PersonEnum::Patient(p) => p.gender,
            PersonEnum::Doctor(d) => d.gender,
        }
    }
    pub fn get_condition(&self) -> Condition {
        match self {
            PersonEnum::Patient(p) => p.condition,
            PersonEnum::Doctor(d) => d.condition,
        }
    }
    pub fn get_mrn(&self) -> &MRN {
        match self {
            PersonEnum::Patient(p) => &p.mrn,
            PersonEnum::Doctor(d) => &d.mrn,
        }
    }
}

impl fmt::Display for PersonEnum {
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
            PersonEnum::Doctor(d) => {
                result.push_str(&format!("Specialty: {:?}\n", d.get_specialty()));
            }
            PersonEnum::Patient(p) => {
                result.push_str(&format!("Notes: {:?}\n", p.get_notes()));
            }
        }
        write!(f, "{result}")
    }
}

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
        NUM_GEN.lock().unwrap().free(self.mrn.clone());
    }
}
