#[derive(Debug, Copy, Clone)]
pub enum Specialty {
    Allergist,
    Anesthesiologists,
    Cardiologists,
    Dermatologists,
    Endocrinologists,
    Hematologists,
    Internists,
    Nephrologists,
    Oncologists,
    Osteopaths,
    Pathologists,
    Pediatricians,
    Physiatrists,
}

pub struct Doctor {
    specialty: Specialty,
}

impl Doctor {
    pub fn new(
        name: String,
        age: i32,
        gender: super::Gender,
        specialty: Specialty,
    ) -> Result<super::PersonE, String> {
        Ok(super::PersonE::Doctor(super::Person::new(
            name,
            age,
            gender,
            super::Condition::Employee,
            Doctor { specialty },
        )?))
    }
}

impl super::Person<Doctor> {
    pub fn get_specialty(&self) -> Specialty {
        self.kind.specialty
    }
}
