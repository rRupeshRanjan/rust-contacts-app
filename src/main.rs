pub mod contacts_service;
use contacts_service::contact_error::*;
use contacts_service::domain::*;
use std::net::SocketAddr;
use warp::{self, http::Response, http::StatusCode, Filter};

use crate::contacts_service::ContactsService;
use crate::contacts_service::SqlContactsService;

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

async fn handle_read_response(
    response: Result<String, ContactsError>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match response {
        Ok(response_body) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(response_body)),
        Err(ContactsError::NotFoundError(err)) => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(err.to_string())),
        Err(err) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(err.to_string())),
    }
}

async fn handle_delete_response(
    response: Result<String, ContactsError>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match response {
        Ok(_) => Ok(Response::builder().status(StatusCode::NO_CONTENT).body("")),
        Err(_) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Internal server error")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let contacts_service =
        warp::any().map(|| SqlContactsService::new().expect("Failed to create SqlContactsService"));

    let get_all_contacts_route = warp::path("contacts")
        .and(warp::get())
        .and(warp::query::<GetAllQueryParams>())
        .and(contacts_service.clone())
        .and_then(
            |params: GetAllQueryParams, mut contacts_service: SqlContactsService| {
                let response = contacts_service.get_all(params.page_num, params.page_size);
                handle_read_response(response)
            },
        );

    let get_contact_by_name_route = warp::path!("contacts" / String)
        .and(warp::get())
        .and(contacts_service.clone())
        .and_then(|name: String, mut contacts_service: SqlContactsService| {
            let response = contacts_service.get_by_name(name);
            handle_read_response(response)
        });

    let delete_contact_route = warp::path!("contacts" / String)
        .and(warp::delete())
        .and(contacts_service.clone())
        .and_then(|name: String, mut contacts_service: SqlContactsService| {
            let response = contacts_service.delete(name);
            handle_delete_response(response)
        });

    let create_contact_route = warp::path("contact")
        .and(warp::post())
        .and(warp::body::json())
        .and(contacts_service.clone())
        .and_then(
            |contact: Contact, mut contacts_service: SqlContactsService| {
                let response = contacts_service.add(
                    contact.name.clone(),
                    contact.email,
                    contact.phone_number.to_string(),
                );
                handle_create_update_response(contacts_service, contact.name, response)
            },
        );

    let update_email_route = warp::path("update_email")
        .and(warp::put())
        .and(warp::body::json())
        .and(contacts_service.clone())
        .and_then(
            |contact: UpdateEmailBody, mut contacts_service: SqlContactsService| {
                let response = contacts_service.update_email(contact.name.clone(), contact.email);
                handle_create_update_response(contacts_service, contact.name, response)
            },
        );

    let update_phone_number_route = warp::path("update_phone_number")
        .and(warp::put())
        .and(warp::body::json())
        .and(contacts_service.clone())
        .and_then(
            |contact: UpdatePhoneNumberBody, mut contacts_service: SqlContactsService| {
                let response = contacts_service
                    .update_phone(contact.name.clone(), contact.phone_number.to_string());
                handle_create_update_response(contacts_service, contact.name, response)
            },
        );

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
