#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

fn main() -> Result<(), &'static str> {
   find_by_name("jose");

   Ok(())
}

fn find_by_name(name: &str) -> Result<Person, &'static str> {
    let people = vec![
        Person {
            name: String::new("john"),
            age: 18
        },
        Person {
            name: String::new("jose"),
            age: 20
        }          
    ];
    let found_person = people.iter().find(|person| { person.name.eq(name) });

    dbg!(found_person);
}
