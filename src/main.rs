pub mod contacts_service;
use contacts_service::contact::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::usize;
use warp::{self, Filter};

use crate::contacts_service::ContactsService;
use crate::contacts_service::SqlContactsService;

#[derive(Deserialize, Serialize)]
struct GetAllQueryParams {
    page_num: usize,
    page_size: usize,
}

async fn get_all_contacts(
    page_num: usize,
    page_size: usize,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut contacts = SqlContactsService::new().expect("Failed to create SqlContactsService");
    match contacts.get_all(page_num, page_size) {
        Ok(contacts) => Ok(contacts),
        Err(_) => return Err(warp::reject::not_found()),
    }
}

async fn get_contacts_by_name(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let mut contacts = SqlContactsService::new().expect("Failed to create SqlContactsService");
    match contacts.get_by_name(name) {
        Ok(contact) => Ok(contact),
        Err(_) => return Err(warp::reject::not_found()),
    }
}

async fn delete_contact(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let mut contacts = SqlContactsService::new().expect("Failed to create SqlContactsService");
    match contacts.delete(name) {
        Ok(_) => Ok("Contact deleted"),
        Err(_) => return Err(warp::reject::not_found()),
    }
}

async fn create_contact(contact: Contact) -> Result<impl warp::Reply, warp::Rejection> {
    let mut contacts = SqlContactsService::new().expect("Failed to create SqlContactsService");
    match contacts.add(
        contact.name.clone(),
        contact.email.clone(),
        contact.phone_number.clone().to_string(),
    ) {
        Ok(_) => match contacts.get_by_name(contact.name) {
            Ok(contact) => Ok(contact),
            Err(_) => return Err(warp::reject::not_found()),
        },
        Err(_) => return Err(warp::reject::not_found()),
    }
}

async fn update_email(contact: UpdateEmailBody) -> Result<impl warp::Reply, warp::Rejection> {
    let mut contacts = SqlContactsService::new().expect("Failed to create SqlContactsService");
    match contacts.update_email(contact.name.clone(), contact.email.clone()) {
        Ok(_) => match contacts.get_by_name(contact.name) {
            Ok(contact) => Ok(contact),
            Err(_) => return Err(warp::reject::not_found()),
        },
        Err(_) => return Err(warp::reject::not_found()),
    }
}

async fn update_phone_number(
    contact: UpdatePhoneNumberBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut contacts = SqlContactsService::new().expect("Failed to create SqlContactsService");
    match contacts.update_phone(
        contact.name.clone(),
        contact.phone_number.clone().to_string(),
    ) {
        Ok(_) => match contacts.get_by_name(contact.name) {
            Ok(contact) => Ok(contact),
            Err(_) => return Err(warp::reject::not_found()),
        },
        Err(_) => return Err(warp::reject::not_found()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let get_all_contacts_route = warp::path("contacts")
        .and(warp::get())
        .and(warp::query::<GetAllQueryParams>())
        .and_then(|params: GetAllQueryParams| get_all_contacts(params.page_num, params.page_size));

    let get_contact_by_name_route = warp::path!("contacts" / String)
        .and(warp::get())
        .and_then(|name| get_contacts_by_name(name));

    let delete_contact_route = warp::path!("contacts" / String)
        .and(warp::delete())
        .and_then(|name| delete_contact(name));

    let create_contact_route = warp::path("contact")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create_contact);

    let update_email_route = warp::path("update_email")
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update_email);

    let update_phone_number_route = warp::path("update_phone_number")
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update_phone_number);

    let routes = get_all_contacts_route
        .or(get_contact_by_name_route)
        .or(delete_contact_route)
        .or(create_contact_route)
        .or(update_email_route)
        .or(update_phone_number_route);

    let address: SocketAddr = "127.0.0.1:8000".parse().unwrap();
    warp::serve(routes).bind(address).await;

    Ok(())
}
