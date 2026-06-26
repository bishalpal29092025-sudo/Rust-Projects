mod input;
mod models;
mod statistics;
mod student;

use input::read_u32;
use models::Student;
use student::show_menu;

fn main() {
    let students: Vec<Student> = Vec::new();

    loop {
        show_menu();

        let choice = read_u32("Choose an option: ");

        match choice {
            1 => {
                println!("Add Student (Coming Soon)");
            }

            2 => {
                println!("View Students (Coming Soon)");
            }

            3 => {
                println!("Search Student (Coming Soon)");
            }

            4 => {
                println!("Update Student (Coming Soon)");
            }

            5 => {
                println!("Delete Student (Coming Soon)");
            }

            6 => {
                println!("Statistics (Coming Soon)");
            }

            7 => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid option."),
        }
    }

    drop(students);

}
