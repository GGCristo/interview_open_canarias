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

pub struct Person {
    name: String,
    age: i32,
    gender: Gender,
    condition: Condition,
}

pub trait PersonTrait {
    fn get_person(&self) -> &Person;
    fn get_name(&self) -> &String {
        &self.get_person().name
    }
    fn get_age(&self) -> i32 {
        self.get_person().age
    }
    fn get_gender(&self) -> Gender {
        self.get_person().gender
    }
    fn get_status(&self) -> Condition {
        self.get_person().condition
    }
}
