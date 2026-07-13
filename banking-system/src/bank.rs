use crate::input::{read_f64, read_string};
use crate::models::Account;

pub fn show_menu() {
    println!();
    println!("=================================");
    println!("      Banking System");
    println!("=================================");
    println!("1. Create Account");
    println!("2. View Accounts");
    println!("3. Search Account");
    println!("4. Deposit Money");
    println!("5. Withdraw Money");
    println!("6. Transfer Money");
    println!("7. Delete Account");
    println!("8. Bank Statistics");
    println!("9. Exit");
}

pub fn create_account(
    accounts: &mut Vec<Account>,
    next_account_id: &mut u32,
) {
    println!("\n===== Create Account =====");

    let name = read_string("Enter Account Holder Name:");

    let balance = read_f64("Enter Initial Balance:");

    let account = Account {
        id: *next_account_id,
        name,
        balance,
    };

    accounts.push(account);

    println!("\n✅ Account Created Successfully!");
    println!("Account ID: {}", *next_account_id);

    *next_account_id += 1;
}

pub fn view_accounts(accounts: &Vec<Account>) {
    if accounts.is_empty() {
        println!("No accounts found.");
        return;
    }

    println!();

    println!(
        "{:<5} {:<25} {:<15}",
        "ID",
        "Account Holder",
        "Balance"
    );

    println!("{}", "-".repeat(50));

    for account in accounts {
        println!(
            "{:<5} {:<25} ₹{:<15.2}",
            account.id,
            account.name,
            account.balance
        );
    }
}