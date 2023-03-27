use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub phone_number: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateEmailBody {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePhoneNumberBody {
    pub name: String,
    pub phone_number: i64,
}
