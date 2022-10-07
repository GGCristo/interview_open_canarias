pub struct Patient {
    notes: Vec<String>,
}

// pub fn new(
//     name: String,
//     age: i32,
//     gender: super::Gender,
//     condition: super::Condition,
//     notes: Vec<String>,
// ) -> super::PersonS<Patient> {
//     super::new(name, age, gender, condition, Patient { notes })
// }

pub fn new(
    name: String,
    age: i32,
    gender: super::Gender,
    condition: super::Condition,
    notes: Vec<String>,
) -> super::Person {
    super::Person::Patient(super::new(name, age, gender, condition, Patient { notes }))
}

impl super::PersonS<Patient> {
    pub fn get_notes(&self) -> &Vec<String> {
        &self.kind.notes
    }
}
