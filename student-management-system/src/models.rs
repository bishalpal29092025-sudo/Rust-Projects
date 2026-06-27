#[derive(Debug, Clone, PartialEq)]
pub enum Grade {
    A,
    B,
    C,
    D,
    F,
}


#[derive(Debug, Clone)]
pub struct Student{
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub marks: f32,
    pub grade: Grade,
}
