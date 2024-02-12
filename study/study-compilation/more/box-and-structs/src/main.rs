fn heap_only_values() {
    // lets say I want to store a specific value in the heap by default
    // even if it's only a integer or an even simpler type.

    // To accomplish this task you can use the Box type
    let example = Box::new(32);

    // Why doe? yes, my example is questionable, but I'm sure it has a lot of
    // use cases, just like Mutex and Arc. The only way to know that is getting
    // more knowledge in the language and diving deeper to the more advanced use cases
}

fn struct_example() {
    #[derive(Debug)]
    struct Person {
        name: String,
        last_name: String,
        age: u8,
    }

    let person = Person {
        name: "some name".to_owned(),
        last_name: "some last name".to_owned(),
        age: 21,
    };

    println!("{person:?}");
}

fn impl_struct_example() {
    struct Printer {
        value: String,
    }

    impl Printer {
        fn new(value: &str) -> Self {
            Self {
                value: value.to_owned(),
            }
        }

        fn print(self: &Self) {
            println!("{}", self.value);
        }
    }

    let to_be_printed = Printer::new("I wanna be printed!");
    to_be_printed.print();
}

fn struct_like_enum() {
    struct Coords(u32, u32);

    let Coords(x, y) = Coords(32, 32);
}
