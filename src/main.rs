mod person;

use person::{doctor, patient, Person};

fn print_all_age(persons: &Vec<Person>) {
    for person in persons {
        println!("Age {}", person.get_age());
        match person {
            self::Person::Doctor(d) => println!("Specialty: {:?}", d.get_specialty()),
            self::Person::Patient(p) => println!("Notes: {:#?}", p.get_notes()),
        }
    }
}

const AGE: i32 = 22;
fn main() {
    let maria = patient::new(
        "maria".to_string(),
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
    // foo(&maria);
    // foo(&pepe);
    let mut v: Vec<Person> = vec![];
    v.push(maria);
    v.push(pepe);
    // v.push(Box::new(maria));
    // v.push(Box::new(pepe));
    print_all_age(&v);
}
