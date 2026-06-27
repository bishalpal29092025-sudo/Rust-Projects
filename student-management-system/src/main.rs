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

    4 => student::update_student(&mut students),

    5 => student::delete_student(&mut students),

    6 => statistics::show_statistics(&students),

    7 => student::sort_students(&mut students),

    8 => student::filter_students(&students),

    9 => {
        println!("Goodbye!");
        break;
    }

    _ => println!("Invalid Option"),
}
    }
}
