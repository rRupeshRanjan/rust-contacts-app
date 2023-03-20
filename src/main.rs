pub mod contacts_service;
use crate::contacts_service::{ContactsService, InMemoryConytactsService};

fn main() {
    let name = "Rupesh".to_string();
    let email = "rupesh@mta.com".to_string();
    let phone_number = "311234567890".to_string();
    let mut contacts = InMemoryConytactsService::new();

    let _ = contacts.add(name.clone(), email.clone(), phone_number.clone());
    println!("{:?}", contacts);

    let _ = contacts.update_email(name.clone(), "rup@met.com".to_string());
    println!("{:?}", contacts);

    let _ = contacts.update_phone(name.clone(), "311234567891".to_string());
    println!("{:?}", contacts);

    match contacts.get_by_name(name.clone()) {
        Some(contact) => {
            println!("{:?}", contact)
        }
        None => println!("No contact found by name: {}", name.clone()),
    }

    match contacts.get_by_name("rakesh".to_string()) {
        Some(contact) => {
            println!("{:?}", contact)
        }
        None => println!("No contact found by name: {}", "rakesh".to_string()),
    }

    println!("{:?}", contacts.get_all(0, 1));
}
