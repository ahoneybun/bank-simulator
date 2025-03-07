use std::io;

fn welcome_message() {
    println!("Welcome to Thelio Bank Inc!")
}

fn create_account() {
    println!();
    println!("Please provide your complete name:");

    let mut account_name = String::new();

    io::stdin()
        .read_line(&mut account_name)
        .expect("Failed to read line");

    println!();
    println!("Welcome {}", account_name);
}

fn main() {
    println!();
    welcome_message();
    create_account();

    println!("How much are you depositing?");

    let mut balance_input = String::new();

    io::stdin()
        .read_line(&mut balance_input)
        .expect("Failed to read line");

    let x: i128 = balance_input.trim().parse().expect("Input is not an interger");}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {

}
