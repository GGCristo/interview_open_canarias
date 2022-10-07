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

struct Person {
    name: String,
    age: i32,
    gender: Gender,
    condition: Condition,
}

pub trait PersonTrait {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> i32;
    fn get_gender(&self) -> Gender;
    fn get_status(&self) -> Condition;
}
