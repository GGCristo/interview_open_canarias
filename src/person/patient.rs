pub struct Patient {
    notes: Vec<String>,
}

impl Patient {
    pub fn new(
        name: String,
        age: i32,
        gender: super::Gender,
        condition: super::Condition,
        notes: Vec<String>,
    ) -> Result<super::PersonE, String> {
        Ok(super::PersonE::Patient(super::Person::new(
            name,
            age,
            gender,
            condition,
            Patient { notes },
        )?))
    }
}

impl super::Person<Patient> {
    pub fn get_notes(&self) -> &Vec<String> {
        &self.kind.notes
    }
}
