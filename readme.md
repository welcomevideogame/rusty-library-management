# Rusty Library Management
- **A lightweight library management system**

## Setting Up
- Make sure you have Rust
- Clone repository
- Set up a database with REST API support
  - [Supabase](https://supabase.com)
  - [Firebase](https://firebase.google.com)

## Dependencies
- Install with: `cargo install`

## Usage
- To run this project
> cargo run


## Database Requirements
- Tables must be set up in this format
- Modify table policy to allow permissions for API
  - Employee
  - Media
  - *...more to come*
- You can add random text to the beginning of your tables
  - Example -> random_Employee
  - Specify this in the **"salt"** setting in the config.ini
    - Would be "random_" in this setting

## Contributing
Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change

## Common Questions
- **Q:** Why am I getting nothing when trying to grab data from the database
  - **A:** You most likely do not have the policy modified correctly for the table