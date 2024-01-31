use std::collections::HashMap;

#[derive(Debug)]
struct Locker {
   number: u8,
   content: String
}

fn main() {
    let mut lockers = HashMap::new();

    lockers.insert(0, Locker {
        number: 0,
        content: String::from("bla"),
    });

    lockers.insert(1, Locker {
        number: 1,
        content: String::from("bla"),
    });

    for (key, value) in lockers.iter() {
        println!("{key:?}: {value:?}");
    }
}
