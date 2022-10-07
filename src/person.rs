pub mod doctor;
pub mod patient;

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

pub struct PersonS<T> {
    name: String,
    age: i32,
    gender: Gender,
    condition: Condition,
    kind: T,
}

pub enum Person {
    Patient(PersonS<patient::Patient>),
    Doctor(PersonS<doctor::Doctor>),
}

fn new<T>(name: String, age: i32, gender: Gender, condition: Condition, kind: T) -> PersonS<T> {
    PersonS {
        name,
        age,
        gender,
        condition,
        kind,
    }
}

impl<T> PersonS<T> {
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_age(&self) -> i32 {
        self.age
    }
    pub fn get_gender(&self) -> Gender {
        self.gender
    }
    pub fn get_status(&self) -> Condition {
        self.condition
    }
}
