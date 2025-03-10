use std::io;

fn welcome_message() {
    println!("Welcome to Thelio Bank Inc!");
    println!();
}

fn main() {
    println!();
    welcome_message();

    let account = BankAccount {
        owner: "Aaron Honeycutt".to_string(),
        account_number: 123456789,
        balance: 145.50,
        pin: 1234,

    };
   
    println!("Welcome {}", account.owner);
    println!();

    println!("What would you like to set your PIN to?");
    account.set_pin();
    println!();
}

struct BankAccount {
    owner: String,
    account_number: i128,
    balance: f64,
    pin: u16,
}

impl BankAccount {
    // fn desposit(&mut self, amount: f64) {
    //     println!("Depositing {} to account ending in {}", amount, self.account_number);
    //     self.balance += amount;
    // }

    fn check_balance(&self) {
        println!("your current balance is {} in account ending in {}", self.balance, self.account_number);
    }

    fn set_pin(&self) {
        let mut pin = String::new();

        io::stdin()
            .read_line(&mut pin)
            .expect("Failed to read line");
    
        println!("You have set your pin successfully.");
    }
}
