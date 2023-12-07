# Rusty Library Management
- **A lightweight library management system**
- **Built with Tauri**

# Features
- Blazing speeds
- Great UI while being light on resources!
- Management for library items such as
    - Employees
    - Media
- Secure login system
    - Implements award-winning [Argon2](https://www.password-hashing.net/) hashing for maximum security
- Cross-platform compatability: Windows, MacOS, and Linux!
- Environmentally friendly: say goodbye to the need for paper

## Setting Up
- Clone repository
- Set up a database with REST API support
  - [Supabase](https://supabase.com)
  - [Firebase](https://firebase.google.com)

## Prerequisites
- Rust
- Node.js

## Dependencies
- Install with: `cargo install` and `npm install`

## Usage
- To run this project
> npm run tauri dev


## Database Requirements
- Tables must be set up in this format
- Modify table policy to allow permissions for API
  - Employee
  - Media
  - *...more to come*
- You can add random salting to the beginning of your tables
  - Example -> random_Employee
  - Specify this in the **"salt"** setting in the config.ini
    - Would be "random_" in this setting

## Contributing
- Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change
- Interested in contributing? Look below for some ideas!

## Future Features
- High customization
    - Customizable user roles and permissions
- Data analytics

## Common Questions
- **Q:** Why am I getting nothing when trying to grab data from the database
  - **A:** You most likely do not have the policy modified correctly for the table