trait Log {
    fn get_name(&self) -> String;

    // Without the self, it would work the same way as with IMPL
    // it can also have a default implementation
    fn display_info(&self) {
        println!("default");
    } 
}

struct Example {
    name: String
}

struct Example2 {
    name: String
}

impl Log for Example {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Log for Example2 {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn display_info(&self) {
        println!("{}", self.name);   
    }
}

fn main() {
    let example = Example {name: "im a default bitch".to_owned()};
    let example2 = Example2 {name: "im not default".to_owned()};

    example.display_info();
    example2.display_info();
}

// What does impl do? it implements as many versions of the function as structs 
// implementing this trait in the code, (example, example2) and it happens at
// compile time!
fn has_log_trait(value: impl Log) {
    value.display_info();
}

// what does it do and what's the difference? this big ol' guy uses dynamic dispatch during runtime
// to decide which function it needs to call, making it a lower bundle but relying on runtime...
fn has_log_trait_dyn(value: &dyn Log) {
    value.display_info();
}
