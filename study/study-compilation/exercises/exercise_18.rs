use core::fmt;

#[derive(Debug)]
struct Adult<'a> {
    age: u8,
    name: &'a str,
}

impl fmt::Display for Adult<'_> {
    fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f.debug_struct("Adult")
        //     .field("pog", &self.age)
        //     .field("pog2", &self.age)
        //     .finish()

        // WHY WHEN I CAN JUST \/

        write!(f, "An adult named {} is {}yo", self.name, self.age)
    }
}

impl Adult<'_> {
    fn new(name: &'static str, age: u8) -> Result<Self, &str> {
        if age < 21 {
            return Err("Not old enough buddy");
        }

        Ok(Self { name, age })
    }
}

fn main() -> Result<(), &'static str> {
    let person_01 = Adult::new("JOHN", 21)?;
    let mut person_02 = Adult::new("JONATHAN", 20);

    println!("{person_01}");

    if let Err(value) = person_02 {
        println!("{value}");

        person_02 = Adult::new("JONATHAN", 21); // borrowing his friend's id cof cof
    }

    println!("{}", person_02?);

    Ok(())
}
