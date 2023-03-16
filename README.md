# rust-contacts-app

How to run:

- Go to root folder
- Run "cargo run", whcih will prompt below:
    Choose your action -
    1. Add to contacts
    2. Edit contact
    3. Delete contact
    4. See all contacts
    5. See contact by name
- Choose Action 1 to add contact with prompts:
    - Enter name:
    - Enter email: (This will validate email, and if invalid, will re-prompt same with warning that email is wrong)
    - Enter contact number: (Same logic as above)
- Choose Action 2 to edit contact with prompts:
    - Enter name: (if name does not exist, it aborts. otherwise below prompts)
    - Enter email: (This will validate email, and if invalid, will re-prompt same with warning that email is wrong)
    - Enter contact number: (Same logic as above)
- Choose Action 3 to delete contact with prompts:
    - Enter name: (if copntact by name exists, it deletes, otherwise prompts taht this does not exist)
- Choose Action 4 to see all contacts, 3 per page, where the prompts will ask to input page number
    - Enter page number: (The page number starts from 1)
- Choose Action 5 to search contact by name
    - Enter name: (Prints contact, if exists, otehrwise tells user that this does not exist)
- After every choice is done, you get a prompt if you want to continue the show:
    - "Do you want to coninue?(y/n)" (Repeats all above if "y", else terminates the execution)
