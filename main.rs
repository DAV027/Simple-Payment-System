use std::collections::HashMap;
use std::io::{self, Write};
use stellar_sdk::SecretKey;
use chrono::Utc;

#[derive(Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
    timestamp: String,
}

struct PaymentSystem {
    balances: HashMap<String, f64>,
    transaction_history: Vec<Transaction>,
}

impl PaymentSystem {
    fn new() -> PaymentSystem {
        PaymentSystem {
            balances: HashMap::new(),
            transaction_history: Vec::new(),
        }
    }

    fn create_wallet(&mut self) {
        let keypair = SecretKey::random();
        let public_key = keypair.public_key().to_encoding();
        let secret_key = keypair.to_encoding();
        self.balances.insert(public_key.clone(), 0.0);
        println!("New wallet created successfully!");
        println!("Public Key: {}", public_key);
        println!("Secret Key: {}", secret_key);
    }

    fn check_balance(&self, public_key: &str) {
        let balance = self.balances.get(public_key).unwrap_or(&0.0);
        println!("Balance: {}", balance);
    }

    fn add_balance(&mut self, public_key: &str, amount: f64) {
        let balance = self.balances.entry(public_key.to_string()).or_insert(0.0);
        *balance += amount;
        println!("Balance added successfully!");
    }

    fn send_payment(&mut self, sender_secret_key: &str, receiver_public_key: &str, amount: f64) {
        let sender_keypair = match SecretKey::from_encoding(sender_secret_key) {
            Ok(kp) => kp,
            Err(_) => {
                println!("Invalid secret key.");
                return;
            }
        };
        let sender_public_key = sender_keypair.public_key().to_encoding();

        let sender_balance = self.balances.get_mut(&sender_public_key);
        match sender_balance {
            Some(balance) if *balance >= amount => {
                *balance -= amount;
                let receiver_balance = self.balances.entry(receiver_public_key.to_string()).or_insert(0.0);
                *receiver_balance += amount;
                let transaction = Transaction {
                    sender: sender_public_key,
                    receiver: receiver_public_key.to_string(),
                    amount,
                    timestamp: Utc::now().to_rfc3339(),
                };
                self.transaction_history.push(transaction);
                println!("Payment sent successfully!");
            }
            _ => {
                println!("Insufficient balance to send payment.");
            }
        }
    }

    fn display_transaction_history(&self) {
        println!("\nTransaction History:");
        for (index, transaction) in self.transaction_history.iter().enumerate() {
            println!("Transaction {}:", index + 1);
            println!("Sender: {}", transaction.sender);
            println!("Receiver: {}", transaction.receiver);
            println!("Amount: {}", transaction.amount);
            println!("Timestamp: {}", transaction.timestamp);
            println!("--------------------------");
        }
    }
}

fn main_menu(system: &mut PaymentSystem) {
    loop {
        println!("\nSimple Payment System");
        println!("1. Create Wallet");
        println!("2. Check Balance");
        println!("3. Add Balance");
        println!("4. Send Payment");
        println!("5. Transaction History");
        println!("6. Exit");
        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();
        let option = option.trim();

        match option {
            "1" => {
                system.create_wallet();
            }
            "2" => {
                print!("Enter your public key: ");
                io::stdout().flush().unwrap();
                let mut public_key = String::new();
                io::stdin().read_line(&mut public_key).unwrap();
                let public_key = public_key.trim();
                system.check_balance(public_key);
            }
            "3" => {
                print!("Enter your public key: ");
                io::stdout().flush().unwrap();
                let mut public_key = String::new();
                io::stdin().read_line(&mut public_key).unwrap();
                let public_key = public_key.trim();

                print!("Enter amount to add: ");
                io::stdout().flush().unwrap();
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).unwrap();
                let amount: f64 = amount.trim().parse().expect("Invalid amount");

                system.add_balance(public_key, amount);
            }
            "4" => {
                print!("Enter your secret key: ");
                io::stdout().flush().unwrap();
                let mut sender_secret_key = String::new();
                io::stdin().read_line(&mut sender_secret_key).unwrap();
                let sender_secret_key = sender_secret_key.trim();

                print!("Enter recipient public key: ");
                io::stdout().flush().unwrap();
                let mut receiver_public_key = String::new();
                io::stdin().read_line(&mut receiver_public_key).unwrap();
                let receiver_public_key = receiver_public_key.trim();

                print!("Enter amount to send: ");
                io::stdout().flush().unwrap();
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).unwrap();
                let amount: f64 = amount.trim().parse().expect("Invalid amount");

                system.send_payment(sender_secret_key, receiver_public_key, amount);
            }
            "5" => {
                system.display_transaction_history();
            }
            "6" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}

fn main() {
    let mut system = PaymentSystem::new();
    main_menu(&mut system);
}
