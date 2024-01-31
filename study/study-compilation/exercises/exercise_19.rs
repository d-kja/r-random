#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u8,
}

fn main() -> Result<(), &'static str> {
    find_by_name("jose");

    Ok(())
}

fn find_by_name(name: &str) -> Result<Person, &'static str> {
    let people = vec![
        Person {
            name: String::from("john"),
            age: 18,
        },
        Person {
            name: String::from("jose"),
            age: 20,
        },
    ];
    let found_person = match people
        .iter()
        .find(|&person| person.name.eq(name)) {
             Some(value) => value.clone(),
             None => return Err("not found.")
        }; 

    dbg!(&found_person);

    Ok(found_person)
}
