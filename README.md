# rust-contacts-app

How to run:

- Run "cargo run", which exposes below rest apis:
    - GET /contacts?page_num=<>&page_size=<>
    - GET /contact/<name>
    - DELETE /contact/<name>
    - POST /contact
        - Body: {"name" : "some_name", "email": "some-email@abc.com", "phone_number": 311234567890}
    - PUT /update_email
        - Body: {"name" : "some_name", "email": "some-email@abc.com"}
    - PUT /update_phone_number
        - Body: {"name" : "some_name", "phone_number": 311234567890}
