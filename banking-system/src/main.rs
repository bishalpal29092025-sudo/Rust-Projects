mod bank;
mod input;
mod models;
mod statistics;
mod transaction;
mod validation;

use input::read_u32;
use models::{Account, Transaction};

fn main() {
    let mut accounts: Vec<Account> = Vec::new();
    let mut transactions: Vec<Transaction> = Vec::new();

    let mut next_account_id = 1;
    let mut next_transaction_id = 1;

    loop {
        bank::show_menu();

        let choice = read_u32("Choose an option:");

        match choice {
            1 => bank::create_account(&mut accounts, &mut next_account_id),

            2 => bank::view_accounts(&accounts),
            3 => println!("Search Account (Coming Soon)"),
            4 => println!("Deposit Money (Coming Soon)"),
            5 => println!("Withdraw Money (Coming Soon)"),
            6 => println!("Transfer Money (Coming Soon)"),
            7 => println!("Delete Account (Coming Soon)"),
            8 => println!("Statistics (Coming Soon)"),

            9 => {
                println!("Thank you for using Banking System!");
                break;
            }

            _ => println!("Invalid option."),
        }
    }

    drop(accounts);
    drop(transactions);
    drop(next_account_id);
    drop(next_transaction_id);
}
