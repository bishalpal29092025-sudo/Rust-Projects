use crate::models::{Grade, Student};

pub fn show_statistics(students: &Vec<Student>) {
    println!();
    println!("==============================");
    println!(" Student Statistics");
    println!("==============================");

    if students.is_empty() {
        println!("No students available.");
        return;
    }

    println!("Total Students : {}", total_students(students));
    println!("Average Marks  : {:.2}", average_marks(students));
    println!("Highest Marks  : {:.2}", highest_marks(students));
    println!("Lowest Marks   : {:.2}", lowest_marks(students));

    println!();
    println!("Grade Distribution");
    println!("------------------");

    println!("A : {}", count_grade(students, Grade::A));
    println!("B : {}", count_grade(students, Grade::B));
    println!("C : {}", count_grade(students, Grade::C));
    println!("D : {}", count_grade(students, Grade::D));
    println!("F : {}", count_grade(students, Grade::F));
}

fn total_students(students: &Vec<Student>) -> usize {
    students.len()
}

fn average_marks(students: &Vec<Student>) -> f32 {
    let total: f32 = students
        .iter()
        .map(|student| student.marks)
        .sum();

    total / students.len() as f32
}

fn highest_marks(students: &Vec<Student>) -> f32 {
    students
        .iter()
        .map(|student| student.marks)
        .fold(f32::MIN, f32::max)
}

fn lowest_marks(students: &Vec<Student>) -> f32 {
    students
        .iter()
        .map(|student| student.marks)
        .fold(f32::MAX, f32::min)
}

fn count_grade(students: &Vec<Student>, grade: Grade) -> usize {
    students
        .iter()
        .filter(|student| student.grade == grade)
        .count()
}