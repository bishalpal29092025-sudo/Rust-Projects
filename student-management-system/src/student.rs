use crate::input::{read_f32, read_string, read_u32, read_u8};
use crate::models::{Grade, Student};

pub fn show_menu() {
    println!();
    println!("==============================");
    println!(" Student Management System");
    println!("==============================");
    println!("1. Add Student");
    println!("2. View Student");
    println!("3. Search Student");
    println!("4. Update Student");
    println!("5. Delete Student");
    println!("6. Statistics");
    println!("7. Exit");
}


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

fn grade_to_string(grade: &Grade) -> &'static str {
    match grade {
        Grade::A => "A",
        Grade::B => "B",
        Grade::C => "C",
        Grade::D => "D",
        Grade::F => "F",
    }
}


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

