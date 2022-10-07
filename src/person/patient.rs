pub struct Patient {
    notes: Vec<String>,
    person: super::Person,
}

pub fn new(
    name: String,
    age: i32,
    gender: super::Gender,
    condition: super::Condition,
    notes: Vec<String>,
) -> Patient {
    Patient {
        person: super::Person {
            name,
            age,
            gender,
            condition,
        },
        notes,
    }
}

impl Patient {
    pub fn get_notes(&self) -> &Vec<String> {
        &self.notes
    }
}

impl super::PersonTrait for Patient {
    fn get_name(&self) -> &String {
        &self.person.name
    }
    fn get_age(&self) -> i32 {
        self.person.age
    }
    fn get_gender(&self) -> super::Gender {
        self.person.gender
    }
    fn get_status(&self) -> super::Condition {
        self.person.condition
    }
}
