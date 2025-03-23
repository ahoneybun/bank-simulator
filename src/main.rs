use std::io;

fn welcome_message() {
    println!("Welcome to Thelio Bank Inc!");
    println!();
}

fn main() {
    println!();
    welcome_message();

    let account = BankAccount {
        owner: "Placeholder".to_string(),
        account_number: 6789,
        balance: 0.50,
        pin: 1234,

    };
   
    println!("What is your name?");
    account.create_account();

    account.check_balance();
}

struct BankAccount {
    owner: String,
    account_number: i128,
    balance: f64,
    pin: u16,
}

impl BankAccount {

    fn create_account(&self) {
        let mut owner = String::new();

        io::stdin()
            .read_line(&mut owner)
            .expect("Not a name");

        println!("");
        println!("Welcome {}", owner);
    }

    fn set_pin(&self) {
        let mut pin = String::new();

        io::stdin()
            .read_line(&mut pin)
            .expect("Not a number");
    
        println!("You have set your pin successfully.");
    }

    fn check_balance(&self) {
        println!("your current balance is {}", self.balance);
    }

    // fn desposit(&mut self, amount: f64) {
    //     println!("Depositing {} to account ending in {}", amount, self.account_number);
    //     self.balance += amount;
    // }
}
