#[derive(Copy, Clone)]
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
    person: super::Person,
    specialty: Specialty,
}

pub fn new(name: String, age: i32, gender: super::Gender, specialty: Specialty) -> Doctor {
    Doctor {
        person: super::Person {
            name,
            age,
            gender,
            condition: super::Condition::Employee,
        },
        specialty,
    }
}

impl Doctor {
    fn get_specialty(&self) -> Specialty {
        self.specialty
    }
}

impl super::PersonTrait for Doctor {
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
