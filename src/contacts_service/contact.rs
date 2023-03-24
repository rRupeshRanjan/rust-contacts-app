use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub phone_number: i64,
}
