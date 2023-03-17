use std::collections::HashMap;
use std::io;

const PAGE_SIZE: usize = 3;
const INPUT_SELECTION_MESSAGE: &str = "Choose your action -
1. Add to contacts
2. Edit contact
3. Delete contact
4. See all contacts
5. See contact by name";

#[derive(Debug, Clone)]
struct Contact {
    name: String,
    email: String,
    contact_number: String,
}

fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();

    // If the email address does not contain exactly one "@" symbol, the function returns false.
    if parts.len() != 2 {
        return false;
    }

    let (username, domain) = (parts[0], parts[1]);

    // If either the username or the domain is empty, or domain does not contain "." the function returns false.
    if username.is_empty() || domain.is_empty() || !domain.contains(".") {
        return false;
    }

    // If the domain starts or ends with a "." character, the function returns false.
    if domain.chars().next().unwrap() == '.' || domain.chars().last().unwrap() == '.' {
        return false;
    }

    // If the username or domain contains any character that is
    // not a letter, a digit, a ".", a "_" or a "-", the function returns false.
    if !username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-')
        || !domain
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.')
    {
        return false;
    }

    true
}

fn is_valid_contact_number(contact_number: &str) -> bool {
    // Check if the contact number has only digits and exactly 10 digits
    contact_number.chars().all(|c| c.is_digit(10)) && contact_number.len() == 10
}

fn take_and_validate_email_input() -> String {
    let mut email;
    loop {
        email = take_input("Enter email: ");
        if !is_valid_email(email.as_str()) {
            println!("Invalid email, please enter again");
            continue;
        }
        break;
    }
    email
}

fn take_and_validate_contact_number_input() -> String {
    let mut contact_number;
    loop {
        contact_number = take_input("Enter contact number: ");
        if !is_valid_contact_number(contact_number.as_str()) {
            println!("Invalid contact number, please enter again");
            continue;
        }
        break;
    }
    contact_number
}

fn add_to_contacts(directory: &mut HashMap<String, Contact>) {
    let contact = Contact {
        name: take_input("Enter name: "),
        email: take_and_validate_email_input(),
        contact_number: take_and_validate_contact_number_input(),
    };
    directory.insert(contact.name.clone(), contact.clone());
    println!(
        "added contact with details -> name: {}, email: {}, contact: {}",
        contact.name, contact.email, contact.contact_number
    );
}

fn update_contacts(directory: &mut HashMap<String, Contact>) {
    let name = take_input("Enter name: ");
    if !directory.contains_key(&name) {
        println!("No contacts by this name");
        return;
    }

    let contact = Contact {
        name: name.clone(),
        email: take_and_validate_email_input(),
        contact_number: take_and_validate_contact_number_input(),
    };

    directory.insert(name.clone(), contact.clone());
    println!(
        "updated contact with details -> name: {}, email: {}, contact: {}",
        contact.name, contact.email, contact.contact_number
    );
}

fn delete_contact(directory: &mut HashMap<String, Contact>) {
    let name = take_input("Enter name: ");
    if let Some(val) = directory.remove(&name) {
        println!("Deleted contact -> {:?}", val);
    } else {
        println!("name not found in directory");
    }
}

fn get_contacts_by_page(directory: &HashMap<String, Contact>) {
    let page_num = take_input("Enter page number: ")
        .parse::<usize>()
        .unwrap_or(1);

    let start_index: usize = (page_num - 1) * PAGE_SIZE;

    let contacts: Vec<&Contact> = directory
        .values()
        .skip(start_index)
        .take(PAGE_SIZE)
        .collect();

    println!("--- Contacts (Page {}) ---", page_num);
    for contact in contacts {
        println!(
            "name: {}, email: {}, contact: {}",
            contact.name, contact.email, contact.contact_number
        );
    }
}

fn get_contact_by_name(directory: &HashMap<String, Contact>) {
    let name = take_input("Enter name: ");
    match directory.get(&name) {
        Some(value) => {
            println!(
                "name: {}, email: {}, contact: {}",
                value.name, value.email, value.contact_number
            );
        }
        None => {
            println!("No contacts by this name");
        }
    }
}

fn take_input(message: &str) -> String {
    println!("{}", message);
    let mut entry = String::new();
    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read line");

    entry.trim().to_string()
}

fn main() {
    let mut directory = HashMap::new();

    loop {
        let input = take_input(INPUT_SELECTION_MESSAGE);

        match input.as_str() {
            "1" => add_to_contacts(&mut directory),
            "2" => update_contacts(&mut directory),
            "3" => delete_contact(&mut directory),
            "4" => get_contacts_by_page(&directory),
            "5" => get_contact_by_name(&directory),
            _ => println!("Invalid input, please enter inputs from 1-5"),
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
