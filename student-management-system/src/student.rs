use crate::input::{read_f32, read_string, read_u32, read_u8};
use crate::models::{Grade, Student};

pub fn show_menu() {
    println!();
    println!("==============================");
    println!(" Student Management System");
    println!("==============================");

    println!("1. Add Student");
    println!("2. View Students");
    println!("3. Search Student");
    println!("4. Update Student");
    println!("5. Delete Student");
    println!("6. Statistics");
    println!("7. Sort Students");
    println!("8. Filter Students");
    println!("9. Exit");
}

// Add Students

pub fn add_student(students: &mut Vec<Student>, next_id: &mut u32) {
    println!("\n===== Add Student =====");

    let name = read_string("Enter Name: ");
    let age = read_u8("Enter Age: ");
    let marks = read_f32("Enter Marks: ");

    let grade = calculate_grade(marks);

    let student = Student {
        id: *next_id,
        name,
        age,
        marks,
        grade,
    };

    students.push(student);

    *next_id += 1;

    println!("Student Added Successfully!");
}

fn calculate_grade(marks: f32) -> Grade {
    if marks >= 90.0 {
        Grade::A
    }else if marks >= 80.0 {
        Grade::B
    }else if marks >= 70.0 {
        Grade::C
    }else if marks >= 60.0 {
        Grade::D
    }else {
        Grade::F
    }
}



// View Students

pub fn view_students(students: &Vec<Student>) {
    if students.is_empty() {
        println!("No students found.");
        return;
    }

    println!();
    println!(
        "{:<5} {:<20} {:<5} {:<10} {:<10}",
        "ID", "Name", "Age", "Marks", "Grade"
    );

    println!("{}", "-".repeat(60));

    for student in students{
        println!(
            "{:<5} {:<20} {:<5} {:<10.2} {:<10}",
            student.id,
            student.name,
            student.age,
            student.marks,
            grade_to_string(&student.grade)
        );
    }
}



fn grade_to_string(grade: &Grade) -> &'static str {
    match grade {
        Grade::A => "A",
        Grade::B => "B",
        Grade::C => "C",
        Grade::D => "D",
        Grade::F => "F",
    }
}



// Search Students

pub fn search_student(students: &Vec<Student>) {
    println!();
    println!("===== Search Student =====");
    println!("1. Search by ID");
    println!("2. Search by Name");

    let choice = read_u32("Choose an Option: ");
    match choice {
        1 => search_by_id(students),
        2 => search_by_name(students),
        _ => println!("Invalid option."),
    }
}

fn search_by_id(students: &Vec<Student>) {
    let id = read_u32("Enter Student ID: ");
    match students.iter().find(|student | student.id == id) {
        Some(student) => display_student(student),
        None => println!("Student not found."),
    }
}

fn search_by_name(students: &Vec<Student>) {
    let name = read_string("Enter Student Name: ");
    match students.iter().find(|student |student.name.eq_ignore_ascii_case(&name))
    {
        Some(student) => display_student(student),
        None => println!("Student not found."),
    }
}

fn display_student(student: &Student){
    println!();
    println!("Student Found");
    println!("--------------------------");
    println!("ID     : {}", student.id);
    println!("Name   : {}", student.name);
    println!("Age    : {}", student.age);
    println!("Marks  : {:.2}", student.marks);
    println!("Grade  : {}", grade_to_string(&student.grade));
}



// Update Student 
pub fn update_student(students: &mut Vec<Student>){
    println!("\n===== Update Student =====");

    let id = read_u32("Enter Student ID: ");
    match students.iter_mut().find(|student | student.id == id) {
        Some(student) => {
            println!("Current Name: {}", student.name);
            println!("Current Age: {}", student.age);
            println!("Current Marks: {:.2}", student.marks);

            student.name = read_string("Enter New Name: ");
            student.age = read_u8("Enter New Age: ");
            student.marks = read_f32("Enter the new Marks: ");

            student.grade = calculate_grade(student.marks);

            println!("\n Student Details Updated Successfully!");
        }
        None => println!("Student not found.")
    }
}


// Delete Students

pub fn delete_student(students: &mut Vec<Student>) {
    println!("\n===== Delete Student =====");

    let id = read_u32("Enter Student ID: ");
    let initial_count = students.len();

    students.retain(|student | student.id != id);

    if students.len() < initial_count {
        println!("Student deleted successfully!");
    } else {
        println!("Student not found.");
    }
}

pub fn sort_students(students: &mut Vec<Student>) {
    println!("\n===== Sort Students =====");

    println!("1. Sort by Name");
    println!("2. Sort by Marks");
    println!("3. Sort by Age");

    let choice = read_u32("Choose an option:");

    match choice {
        1 => {
            students.sort_by(|a, b| a.name.cmp(&b.name));
            println!("Students sorted by name.");
        }

        2 => {
            students.sort_by(|a, b| {
                b.marks.partial_cmp(&a.marks).unwrap()
            });

            println!("Students sorted by marks.");
        }

        3 => {
            students.sort_by_key(|student| student.age);

            println!("Students sorted by age.");
        }

        _ => {
            println!("Invalid option.");
            return;
        }
    }

    println!();

    view_students(students);
}

pub fn filter_students(students: &Vec<Student>) {
    println!("\n===== Filter Students =====");

    println!("1. Grade A");
    println!("2. Above Marks");
    println!("3. Failed Students");

    let choice = read_u32("Choose an option:");

    match choice {
        1 => filter_grade_a(students),
        2 => filter_above_marks(students),
        3 => filter_failed(students),
        _ => println!("Invalid option."),
    }
}

fn filter_grade_a(students: &Vec<Student>) {
    println!("\n===== Grade A Students =====");

    for student in students
        .iter()
        .filter(|student| student.grade == Grade::A)
    {
        display_student(student);
    }
}

fn filter_above_marks(students: &Vec<Student>) {
    let marks = read_f32("Enter minimum marks:");

    println!("\n===== Students Above Marks =====");

    for student in students
        .iter()
        .filter(|student| student.marks >= marks)
    {
        display_student(student);
    }
}

fn filter_failed(students: &Vec<Student>) {
    println!("\n===== Failed Students =====");

    for student in students
        .iter()
        .filter(|student| student.grade == Grade::F)
    {
        display_student(student);
    }
}