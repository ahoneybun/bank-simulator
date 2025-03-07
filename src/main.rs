use std::io;

fn welcome_message() {
    println!("Welcome to Thelio Bank Inc!")
}

fn main() {
    println!();
    welcome_message();

    println!();
    println!("Please provide your complete name:");

    let mut account_name = String::new();

    io::stdin()
        .read_line(&mut account_name)
        .expect("Failed to read line");

    println!();
    println!("Welcome {}", account_name);

}
