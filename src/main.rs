pub mod contacts_service;
use std::io;
use std::usize;

use crate::contacts_service::ContactsService;
use crate::contacts_service::SqlContactsService;

const INPUT_SELECTION_MESSAGE: &str = "Choose your action -
1. Add to contacts
2. Update contact email
3. Update contact number
4. Delete contact
5. See all contacts
6. See contact by name";

fn take_input(message: &str) -> String {
    println!("{}", message);
    let mut entry = String::new();
    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read line");

    entry.trim().to_string()
}

fn main() {
    let mut contacts = SqlContactsService::new().expect("Failed to create SqlContactsService");
    loop {
        let input = take_input(INPUT_SELECTION_MESSAGE);

        match input.as_str() {
            "1" => {
                let name = take_input("Enter name:");
                let email = take_input("Enter email:");
                let phone_number = take_input("Enter phone number:");
                match contacts.add(name, email, phone_number) {
                    Ok(_) => println!("contact added to directory"),
                    Err(err) => println!("{}", err.to_string()),
                }
            }
            "2" => {
                let name = take_input("Enter name:");
                let email = take_input("Enter email:");
                match contacts.update_email(name, email) {
                    Ok(_) => println!("updated email"),
                    Err(err) => println!("{}", err.to_string()),
                }
            }
            "3" => {
                let name = take_input("Enter name:");
                let phone_number = take_input("Enter phone number:");
                match contacts.update_phone(name, phone_number) {
                    Ok(_) => println!("updated phone number"),
                    Err(err) => println!("{}", err.to_string()),
                }
            }
            "4" => {
                let name = take_input("Enter name:");
                match contacts.delete(name) {
                    Ok(_) => println!("Contact deleted"),
                    Err(err) => println!("{}", err.to_string()),
                }
            }
            "5" => {
                let page_num = take_input("Enter page number (starts from 0): ");
                let page_size = take_input("Enter page size: ");

                match page_num.parse::<usize>() {
                    Ok(page_num) => match page_size.parse::<usize>() {
                        Ok(page_size) => match contacts.get_all(page_num, page_size) {
                            Ok(contacts) => {
                                println!("Contact List: Page {}\n{}", page_num + 1, contacts)
                            }
                            Err(err) => println!("Error fetching contacts: {}", err.to_string()),
                        },
                        Err(err) => panic!("{}", err.to_string()),
                    },
                    Err(err) => panic!("{}", err.to_string()),
                }
            }
            "6" => {
                let name = take_input("Enter name:");
                match contacts.get_by_name(name) {
                    Ok(contact) => println!("Contact details: {}", contact),
                    Err(err) => println!("Error fetching congtact: {}", err.to_string()),
                }
            }
            _ => println!("Invalid input, please enter inputs from 1-6"),
        }

        let is_next_entry = take_input("Do you want to coninue?(y/n)");
        match is_next_entry.to_lowercase().as_str() {
            "yes" | "y" => continue,
            "no" | "n" => break,
            _ => {
                println!("Invalid input, please enter 'yes' or 'no'");
                break;
            }
        };
    }
}
