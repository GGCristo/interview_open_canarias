mod person;
mod registry;
mod utils;

use person::{doctor, doctor::Doctor, patient::Patient, PersonEnum};
use utils::num_generator::NumGenerator;

fn print_all_age(persons: &Vec<PersonEnum>) {
    for person in persons {
        println!("Age {}", person.get_age());
        match person {
            PersonEnum::Doctor(d) => println!("Specialty: {:?}", d.get_specialty()),
            PersonEnum::Patient(p) => println!("Notes: {:?}", p.get_notes()),
        }
    }
}

const AGE: i32 = 22;
fn main() {
    let mut registry = registry::new();
    let maria = Patient::new(
        "Maria".to_string(),
        AGE,
        person::Gender::Female,
        person::Condition::GoodCondition,
        vec!["note1".to_string(), "note2".to_string()],
    ).unwrap();
    let pepe = Doctor::new(
        "Pepe".to_string(),
        AGE + 1,
        person::Gender::Male,
        doctor::Specialty::Osteopaths,
    ).unwrap();
    if let Err(e) = registry.add(maria) {
        print!("{e}");
    }
    if let Err(e) = registry.add(pepe) {
        print!("{e}");
    }
}
