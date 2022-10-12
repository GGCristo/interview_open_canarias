#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Condition {
    Good,
    SlightIllness,
    SeriousIllness,
    Critical,
}

pub struct Patient {
    condition: Condition,
    notes: Vec<String>,
}

impl Patient {
    pub fn new(
        name: String,
        age: i32,
        gender: super::Gender,
        condition: Condition,
        notes: Vec<String>,
    ) -> Result<super::Person, String> {
        Ok(super::Person::Patient(super::PersonS::new(
            name,
            age,
            gender,
            Patient { notes, condition },
        )?))
    }
}

impl super::PersonS<Patient> {
    pub fn get_notes(&self) -> &Vec<String> {
        &self.kind.notes
    }
    pub fn get_condition(&self) -> Condition {
        self.kind.condition
    }
}
