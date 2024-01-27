#[derive(Debug)]
struct Student<'a> {
    /// student's name
    name: &'a str,
    /// locker number assigned to the student
    assigned_locker: Option<u16>,
}

fn main() {
    let student_01 = Student {
        name: "John",
        assigned_locker: None,
    };

    let student_02 = Student {
        name: "Jose",
        assigned_locker: Some(0),
    };

    dbg!(&student_01, &student_02);

    match student_01.assigned_locker {
        Some(num) => println!("Student 01 {} - locker: {num}", student_01.name),
        None => println!("Student 01 {} - doens't have a locker", student_01.name),
    }

    match student_02.assigned_locker {
        Some(num) => println!("Student 02 {} - locker: {num}", student_02.name),
        None => println!("Student 02 {} - doens't have a locker", student_02.name),
    }
}
