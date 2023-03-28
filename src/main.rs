pub mod contacts_service;
use contacts_service::contact::*;
use contacts_service::contact_error::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::usize;
use warp::{self, http::Response, http::StatusCode, Filter};

use crate::contacts_service::ContactsService;
use crate::contacts_service::SqlContactsService;

#[derive(Deserialize, Serialize)]
struct GetAllQueryParams {
    page_num: usize,
    page_size: usize,
}

async fn handle_create_update_response(
    mut contacts: SqlContactsService,
    name: String,
    response: Result<(), ContactsError>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match response {
        Ok(_) => match contacts.get_by_name(name) {
            Ok(contact) => Ok(Response::builder().status(StatusCode::OK).body(contact)),
            Err(ContactsError::NotFoundError(err)) => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(err.to_string())),
            Err(err) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(err.to_string())),
        },
        Err(ContactsError::InputError(err)) => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(err)),
        Err(err) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(err.to_string())),
    }
}

async fn get_all_contacts(
    page_num: usize,
    page_size: usize,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut contacts = SqlContactsService::new().expect("Failed to create SqlContactsService");
    match contacts.get_all(page_num, page_size) {
        Ok(contacts) => Ok(Response::builder().status(StatusCode::OK).body(contacts)),
        Err(err) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(err.to_string())),
    }
}

async fn get_contacts_by_name(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let mut contacts = SqlContactsService::new().expect("Failed to create SqlContactsService");
    match contacts.get_by_name(name) {
        Ok(contact) => Ok(Response::builder().status(StatusCode::OK).body(contact)),
        Err(ContactsError::NotFoundError(err)) => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(err.to_string())),
        Err(err) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(err.to_string())),
    }
}

async fn delete_contact(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let mut contacts = SqlContactsService::new().expect("Failed to create SqlContactsService");
    match contacts.delete(name) {
        Ok(_) => Ok(Response::builder().status(StatusCode::NO_CONTENT).body("")),
        Err(_) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Internal server error")),
    }
}

async fn create_contact(contact: Contact) -> Result<impl warp::Reply, warp::Rejection> {
    let mut contacts = SqlContactsService::new().expect("Failed to create SqlContactsService");
    let response = contacts.add(
        contact.name.clone(),
        contact.email,
        contact.phone_number.to_string(),
    );
    handle_create_update_response(contacts, contact.name, response).await
}

async fn update_email(contact: UpdateEmailBody) -> Result<impl warp::Reply, warp::Rejection> {
    let mut contacts = SqlContactsService::new().expect("Failed to create SqlContactsService");
    let response = contacts.update_email(contact.name.clone(), contact.email);
    handle_create_update_response(contacts, contact.name, response).await
}

async fn update_phone_number(
    contact: UpdatePhoneNumberBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut contacts = SqlContactsService::new().expect("Failed to create SqlContactsService");
    let response = contacts.update_phone(contact.name.clone(), contact.phone_number.to_string());
    handle_create_update_response(contacts, contact.name, response).await
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
