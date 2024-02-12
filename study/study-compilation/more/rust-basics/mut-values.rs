fn main() {
    let mut message = "Hello world!";
    print(message);

    message = "Hey there!";
    print(message);
}

fn print(value: &str) -> () {
    println!("{}", value);
}
