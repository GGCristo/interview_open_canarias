mod person;
mod registry;
mod utils;

use person::{doctor, doctor::Doctor, patient::Patient};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut registry = registry::new();
    let maria = Patient::new(
        "Maria".to_string(),
        22,
        person::Gender::Female,
        person::Condition::GoodCondition,
        Vec::new(),
    )?;
    let maria_mrn = registry.add(maria)?.get_mrn().clone();
    registry.remove(&maria_mrn)?;
    let pepe = Doctor::new(
        "Pepe".to_string(),
        23,
        person::Gender::Male,
        doctor::Specialty::Osteopaths,
    )?;
    let matilde = Patient::new(
        "Matilde".to_string(),
        34,
        person::Gender::Other,
        person::Condition::SeriousIllness,
        vec!["note1".to_string(), "note2".to_string()],
    )?;
    registry.add(pepe)?;
    registry.add(matilde)?;
    println!("Registry: {registry}");
    Ok(())
}
