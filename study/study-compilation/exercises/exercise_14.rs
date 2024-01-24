// enum Color {
//     BLACK,
//     WHITE,
//     RED,
//     BLUE,
//     GREEN,
//     PURPLE,
//     CYAN,
// } I GUESS I CAN JUST GO FUCK ME THEN...

struct Person {
    age: u8,
    name: String,
    color: String,
}

impl Person {
    fn new(name: &str, age: u8, color: &str) -> Self {
        Self {
            color: String::from(color),
            name: name.to_owned(),
            age,
        }
    }

    fn print(name: &str, color: &str) {
        println!("{name}, {color}");
    }
}

fn main() {
    let people = vec![
        Person::new("john", 32, "purple"),
        Person::new("juana", 8, "orange"),
        Person::new("bluana", 7, "bluana"), // =D
    ];

    for person in people {
        if person.age <= 10 {
            Person::print(person.name.as_ref(), &person.color)
        }
    }
}
