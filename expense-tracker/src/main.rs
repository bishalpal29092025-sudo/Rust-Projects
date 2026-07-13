use std::io;

#[derive(Debug)]
struct Expense {
    id: u32,
    title: String,
    amount: f64,
}

fn main() {
    let mut expenses: Vec<Expense> = Vec::new();
    let mut next_id = 1;

    loop {
        println!("\n===== Expense Tracker =====");
        println!("1. Add Expense");
        println!("2. View Expenses");
        println!("3. Total Expenses");
        println!("4. Delete Expense");
        println!("5. Exit");

        let choice = read_u32("Choose option:");

        match choice {
            1 => {
                add_expense(&mut expenses, &mut next_id);
            }
            2 => {
                view_expenses(&expenses);
            }
            3 => {
                show_total(&expenses);
            }
            4 => {
                delete_expense(&mut expenses);
            }
            5 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid Option"),
        }
    }
}

fn read_string(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed");

    input.trim().to_string()
}

fn read_u32(prompt: &str) -> u32 {
    loop {
        let input = read_string(prompt);

        match input.parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Enter a valid number"),
        }
    }
}

fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_string(prompt);

        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Enter a valid amount"),
        }
    }
}

fn add_expense(
    expenses: &mut Vec<Expense>,
    next_id: &mut u32,
) {
    let title = read_string("Enter Title:");

    let amount = read_f64("Enter Amount:");

    let expense = Expense {
        id: *next_id,
        title,
        amount,
    };

    expenses.push(expense);

    *next_id += 1;

    println!("Expense Added Successfully");
}

fn view_expenses(expenses: &Vec<Expense>) {
    if expenses.is_empty() {
        println!("No expenses found");
        return;
    }

    println!("\nExpenses:");

    for expense in expenses {
        println!(
            "ID: {} | {} | ₹{}",
            expense.id,
            expense.title,
            expense.amount
        );
    }
}

fn show_total(expenses: &Vec<Expense>) {
    let total: f64 =
        expenses.iter()
                .map(|e| e.amount)
                .sum();

    println!("Total Expense: ₹{}", total);
}

fn delete_expense(
    expenses: &mut Vec<Expense>,
) {
    let id = read_u32("Enter Expense ID:");

    expenses.retain(|expense| expense.id != id);

    println!("Expense Deleted");
}

