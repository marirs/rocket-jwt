#![allow(unused_must_use)]
use std::io::Write;
use rpassword::read_password;
use rocketjwt::{
    backends::Backend,
    db::model::User,
    secure::tokenizer::hash,
};

fn main() -> rocketjwt::Result<()> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_default();
    if database_url.is_empty() {
        println!("Database url empty");
        std::process::exit(1);
    }
    let backend = Backend::new(&database_url).expect("Cannot connect to database");

    let mut username = String::new();
    let mut email = String::new();
    print!("Enter the username (admin): ");
    std::io::stdout().flush();
    std::io::stdin().read_line(&mut username).expect("error: unable to read user input");

    print!("Enter the password: ");
    std::io::stdout().flush();
    let password = read_password().expect("error: unable to read user input");

    print!("Enter the email address: ");
    std::io::stdout().flush();
    std::io::stdin().read_line(&mut email).expect("error: unable to read user input");

    let username = if username.trim().is_empty() {
        "admin".to_string()
    } else {
        username.trim().to_string()
    };
    let password = password.trim().to_string();
    let email = email.trim().to_string();

    backend
        .add_user(User {
            username,
            password: hash(&password),
            is_admin: true,
            email,
            token: None
        })
}