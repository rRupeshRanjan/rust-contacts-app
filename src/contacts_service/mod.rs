pub(crate) mod contact_error;
pub(crate) mod domain;

use rusqlite::{Connection, Result, ToSql};

use domain::Contact;
use regex::Regex;
use serde_json;

use self::contact_error::ContactsError;

const NL_PHONE_NUMBER_REGEX: &str = r"31[0-9]{9,10}";
const EMAIL_REGEX: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
const EMPTY_PARAMS: &[&dyn ToSql] = &[] as &[&dyn ToSql];
const INSERT_CONTACT_QUERY: &str =
    "INSERT INTO contacts (name, email, phone_number) VALUES (?1, ?2, ?3)";
const UPDATE_EMAIL_QUERY: &str = "UPDATE contacts SET email = ?1 WHERE name = ?2";
const UPDATE_PHONE_QUERY: &str = "UPDATE contacts SET phone_number = ?1 WHERE name = ?2";
const DLETE_CONTACT_QUERY: &str = "DELETE FROM contacts WHERE name = ?1";
const GET_BY_NAME_QUERY: &str = "SELECT name, email, phone_number FROM contacts WHERE name = ?1";
const GET_ALL_QUERY: &str = "SELECT name, email, phone_number FROM contacts LIMIT ?1 OFFSET ?2";

pub trait ContactsService {
    fn add(
        &mut self,
        name: String,
        email: String,
        phone_number: String,
    ) -> Result<(), ContactsError>;
    fn update_email(&mut self, name: String, email: String) -> Result<(), ContactsError>;
    fn update_phone(&mut self, name: String, phone_number: String) -> Result<(), ContactsError>;
    fn delete(&mut self, name: String) -> Result<String, ContactsError>;
    fn get_by_name(&mut self, name: String) -> Result<String, ContactsError>;
    fn get_all(&mut self, page_num: usize, page_size: usize) -> Result<String, ContactsError>;
}

pub struct SqlContactsService {
    conn: Connection,
}

impl SqlContactsService {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("../contacts.db")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            phone_number BIGINT NOT NULL
        )",
            EMPTY_PARAMS,
        )?;
        Ok(SqlContactsService { conn })
    }

    fn validate_input(
        name: &String,
        email: Option<&String>,
        phone_number: Option<&String>,
    ) -> Result<(), ContactsError> {
        if name.is_empty() {
            return Err(ContactsError::InputError("Name cannot be empty".to_owned()));
        }

        if let Some(email) = email {
            match Self::is_valid_email(email) {
                Ok(true) => {}
                Ok(false) => {
                    return Err(ContactsError::InputError(format!(
                        "Invalid email entered: {}",
                        email
                    )));
                }
                Err(err) => {
                    return Err(ContactsError::InputError(format!(
                        "Failed to validate email: {}",
                        err
                    )));
                }
            }
        }

        if let Some(phone) = phone_number {
            match Self::is_valid_phone(phone) {
                Ok(true) => {}
                Ok(false) => {
                    return Err(ContactsError::InputError(format!(
                        "Invalid phone number entered: {}",
                        phone
                    )));
                }
                Err(err) => {
                    return Err(ContactsError::InputError(format!(
                        "Failed to validate phone number: {}",
                        err
                    )));
                }
            }
        }

        Ok(())
    }

    fn is_valid_email(text: &String) -> Result<bool, ContactsError> {
        Self::is_valid_regex(text, EMAIL_REGEX)
            .map_err(|_| ContactsError::InputError("Invalid email regex".to_owned()))
    }

    fn is_valid_phone(text: &String) -> Result<bool, ContactsError> {
        Self::is_valid_regex(text, NL_PHONE_NUMBER_REGEX)
            .map_err(|_| ContactsError::InputError("Invalid phone number regex".to_owned()))
    }

    fn is_valid_regex(text: &String, regex: &str) -> Result<bool, regex::Error> {
        match Regex::new(regex) {
            Ok(re) => Ok(re.is_match(text)),
            Err(err) => return Err(err),
        }
    }
}

impl ContactsService for SqlContactsService {
    fn add(
        &mut self,
        name: String,
        email: String,
        phone_number_as_str: String,
    ) -> Result<(), ContactsError> {
        Self::validate_input(&name, Some(&email), Some(&phone_number_as_str))?;

        let mut stmt = self.conn.prepare(INSERT_CONTACT_QUERY)?;
        match stmt.execute(&[name, email, phone_number_as_str]) {
            Ok(_) => {
                println!("contact created");
                Ok(())
            }
            Err(err) => Err(ContactsError::SqliteError(err)),
        }
    }

    fn update_email(&mut self, name: String, email: String) -> Result<(), ContactsError> {
        Self::validate_input(&name, Some(&email), None)?;

        let mut stmt = self.conn.prepare(UPDATE_EMAIL_QUERY)?;
        match stmt.execute(&[email, name.clone()]) {
            Ok(rows_affected) => {
                if rows_affected == 0 {
                    return Err(ContactsError::InputError(format!(
                        "No contact found with name {}",
                        name
                    )));
                }
                println!("email for {} contact updated", rows_affected);
                Ok(())
            }
            Err(err) => Err(ContactsError::SqliteError(err)),
        }
    }

    fn update_phone(&mut self, name: String, phone_number: String) -> Result<(), ContactsError> {
        Self::validate_input(&name, None, Some(&phone_number))?;

        let mut stmt = self.conn.prepare(UPDATE_PHONE_QUERY)?;
        match stmt.execute(&[phone_number, name.clone()]) {
            Ok(rows_affected) => {
                if rows_affected == 0 {
                    return Err(ContactsError::InputError(format!(
                        "No contact found with name {}",
                        name
                    )));
                }
                println!("phone number for {} contact updated", rows_affected);
                Ok(())
            }
            Err(err) => Err(ContactsError::SqliteError(err)),
        }
    }

    fn delete(&mut self, name: String) -> Result<String, ContactsError> {
        let mut stmt = self.conn.prepare(DLETE_CONTACT_QUERY)?;
        match stmt.execute(&[name.clone()]) {
            Ok(_) => {
                println!("contact deleted, name: {}", name);
                Ok("contact deleted".to_string())
            }
            Err(err) => Err(ContactsError::SqliteError(err)),
        }
    }

    fn get_by_name(&mut self, name: String) -> Result<String, ContactsError> {
        let mut stmt = self.conn.prepare(GET_BY_NAME_QUERY)?;
        let mut rows = stmt.query(&[name])?;

        if let Some(row) = rows.next()? {
            let contact = Contact {
                name: row.get(0)?,
                email: row.get(1)?,
                phone_number: row.get(2)?,
            };
            serde_json::to_string(&contact).map_err(|err| ContactsError::SerdeError(err))
        } else {
            Err(ContactsError::NotFoundError(
                "No entries for this name".to_string(),
            ))
        }
    }

    fn get_all(&mut self, page_num: usize, page_size: usize) -> Result<String, ContactsError> {
        let mut stmt = self.conn.prepare(GET_ALL_QUERY)?;

        let contact_iter =
            stmt.query_map(&[page_size as i64, (page_num * page_size) as i64], |row| {
                Ok(Contact {
                    name: row.get(0)?,
                    email: row.get(1)?,
                    phone_number: row.get(2)?,
                })
            })?;

        let contacts: Result<Vec<_>, rusqlite::Error> = contact_iter.collect();
        let contacts = contacts?;
        serde_json::to_string(&contacts).map_err(|err| ContactsError::SerdeError(err))
    }
}
