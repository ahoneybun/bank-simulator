// use std::io;

fn welcome_message() {
    println!("Welcome to Thelio Bank Inc!")
}

fn main() {
    println!();
    welcome_message();

    let mut account = BankAccount {
        owner: "Aaron Honeycutt".to_string(),
        balance: 145.50,
        account_number: 1234
    };
   
    println!("Welcome {}", account.owner);
    account.check_balance();
}

struct BankAccount {
    owner: String,
    account_number: i128,
    balance: f64,
}

impl BankAccount {
    fn desposit(&mut self, amount: f64) {
        println!("Depositing {} to account ending in {}", amount, self.account_number);
        self.balance += amount;
    }

    fn check_balance(&self) {
        println!("your current balance is {} in account ending in {}", self.balance, self.account_number);
    }
}
