use std::{error, fmt};

#[derive(Debug)]
pub enum ContactsError {
    SerdeError(serde_json::Error),
    SqliteError(rusqlite::Error),
    InputError(String),
    NotFoundError(String),
}

#[derive(Debug)]
pub enum NotFoundError {
    NotFound(String),
}

impl fmt::Display for ContactsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContactsError::SerdeError(e) => {
                write!(f, "Failed to serialize/deserialize contact: {}", e)
            }
            ContactsError::SqliteError(e) => write!(f, "SQLite error: {}", e),
            ContactsError::InputError(e) => write!(f, "Failed to parse input: {}", e),
            ContactsError::NotFoundError(e) => write!(f, "No entries: {}", e),
        }
    }
}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotFoundError::NotFound(e) => write!(f, "No entries found: {}", e),
        }
    }
}

impl error::Error for ContactsError {}
impl error::Error for NotFoundError {}

impl From<rusqlite::Error> for ContactsError {
    fn from(e: rusqlite::Error) -> Self {
        ContactsError::SqliteError(e)
    }
}

impl From<serde_json::Error> for ContactsError {
    fn from(e: serde_json::Error) -> Self {
        ContactsError::SerdeError(e)
    }
}

impl From<String> for ContactsError {
    fn from(e: String) -> Self {
        ContactsError::InputError(e)
    }
}

impl From<String> for NotFoundError {
    fn from(e: String) -> Self {
        NotFoundError::NotFound(e)
    }
}
