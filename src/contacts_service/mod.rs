mod contact;
use contact::Contact;
use std::collections::BTreeMap;

use regex::Regex;

const NL_PHONE_NUMBER_REGEX: &str = r"31[0-9]{9,10}";
const EMAIL_REGEX: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";

pub trait ContactsService {
    fn add(&mut self, name: String, email: String, phone_number: String) -> Result<(), String>;
    fn update_email(&mut self, name: String, email: String) -> Result<bool, String>;
    fn update_phone(&mut self, name: String, phone_number: String) -> Result<bool, String>;
    fn delete(&mut self, name: String) -> Option<Contact>;
    fn get_by_name(&mut self, name: String) -> Option<&Contact>;
    fn get_all(&mut self, page_num: usize, page_size: usize) -> Vec<&Contact>;
}

#[derive(Debug)]
pub struct InMemoryConytactsService {
    contacts: BTreeMap<String, Contact>,
}

impl InMemoryConytactsService {
    pub fn new() -> Self {
        InMemoryConytactsService {
            contacts: BTreeMap::new(),
        }
    }

    fn is_valid_email(text: &String) -> Result<bool, regex::Error> {
        Self::is_valid_regex(text, EMAIL_REGEX)
    }

    fn is_valid_phone(text: &String) -> Result<bool, regex::Error> {
        Self::is_valid_regex(text, NL_PHONE_NUMBER_REGEX)
    }

    fn is_valid_regex(text: &String, regex: &str) -> Result<bool, regex::Error> {
        match Regex::new(regex) {
            Ok(re) => Ok(re.is_match(text)),
            Err(err) => return Err(err),
        }
    }
}

impl ContactsService for InMemoryConytactsService {
    fn add(
        &mut self,
        name: String,
        email: String,
        phone_number_as_str: String,
    ) -> Result<(), String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        match Regex::new(NL_PHONE_NUMBER_REGEX) {
            Ok(x) => {
                assert!(x.is_match(&phone_number_as_str))
            }
            Err(_) => panic!("Incorrect phone number provided"),
        }

        match phone_number_as_str.parse::<u64>() {
            Ok(phone_number) => {
                self.contacts.insert(
                    name.clone(),
                    Contact {
                        name: name,
                        email: email,
                        phone_number: phone_number,
                    },
                );
                return Ok(());
            }
            Err(err) => return Err(err.to_string()),
        }
    }

    fn update_email(&mut self, name: String, email: String) -> Result<bool, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        match Self::is_valid_email(&email) {
            Ok(is_valid_email) => {
                if !is_valid_email {
                    return Err("invalid email entered".to_string());
                }
            }
            Err(err) => return Err(err.to_string()),
        }

        match self.contacts.get_mut(&name) {
            Some(contact) => {
                contact.email = email;
                return Ok(true);
            }
            None => Ok(false),
        }
    }

    fn update_phone(&mut self, name: String, phone_number: String) -> Result<bool, String> {
        if name.is_empty() {
            return Err("name cannot be empty".to_string());
        }

        match Self::is_valid_phone(&phone_number) {
            Ok(is_valid_phone) => {
                if !is_valid_phone {
                    return Err("Invalid phone number entered".to_string());
                }
            }
            Err(err) => return Err(err.to_string()),
        }

        match phone_number.parse::<u64>() {
            Ok(new_phone_number) => match self.contacts.get_mut(&name) {
                Some(contact) => {
                    contact.phone_number = new_phone_number;
                    return Ok(true);
                }
                None => return Ok(false),
            },
            Err(err) => return Err(err.to_string()),
        }
    }

    fn delete(&mut self, name: String) -> Option<Contact> {
        self.contacts.remove(&name)
    }

    fn get_by_name(&mut self, name: String) -> Option<&Contact> {
        self.contacts.get(&name)
    }

    fn get_all(&mut self, page_num: usize, page_size: usize) -> Vec<&Contact> {
        self.contacts
            .values()
            .skip(page_num * page_size)
            .take(page_size)
            .collect()
    }
}
