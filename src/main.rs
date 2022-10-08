mod person;
mod registry;

use person::{doctor, patient, PersonTrait};

fn print_all_age(persons: &Vec<Box<dyn person::PersonTrait>>) {
    for person in persons {
        println!("Age {}", person.get_age());
    }
}

const AGE: i32 = 22;
fn main() {
    let maria = patient::new(
        "Maria".to_string(),
        AGE,
        person::Gender::Female,
        person::Condition::GoodCondition,
        vec!["note1".to_string(), "note2".to_string()],
    );
    let pepe = doctor::new(
        "Pepe".to_string(),
        AGE + 1,
        person::Gender::Male,
        doctor::Specialty::Osteopaths,
    );
    println!("Age {}", maria.get_age());
    let mut v = Vec::<Box<dyn person::PersonTrait>>::with_capacity(2);
    v.push(Box::new(maria));
    v.push(Box::new(pepe));
    print_all_age(&v);
}
