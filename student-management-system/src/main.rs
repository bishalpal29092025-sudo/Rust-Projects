mod input;
mod models;
mod statistics;
mod student;

use input::read_u32;
use models::Student;

fn main() {
    let mut students: Vec<Student> = Vec::new();
    let mut next_id: u32 = 1;

    loop {
        student::show_menu();

        let choice = read_u32("Choose an option: ");

        match choice {
            1 => student::add_student(&mut students, &mut next_id),
            2 => student::view_students(&students),
            3 => student::search_student(&students),
            4 => println!("Update Student (Coming Soon)"),
            5 => println!("Delete Student (Coming Soon)"),
            6 => println!("Statistics (Coming Soon)"),

            7 => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid option."),
        }
    }

}
