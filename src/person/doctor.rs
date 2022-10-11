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
    ) -> Result<super::Person, String> {
        Ok(super::Person::Doctor(super::PersonS::new(
            name,
            age,
            gender,
            super::Condition::Employee,
            Doctor { specialty },
        )?))
    }
}

impl super::PersonS<Doctor> {
    pub fn get_specialty(&self) -> Specialty {
        self.kind.specialty
    }
}
