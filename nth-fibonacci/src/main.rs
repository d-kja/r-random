use std::io;
mod utils;

fn main() {
    let base_number = loop {
        println!("Choose a number to get the nth\n");

        let mut base_number = String::new();
        io::stdin().read_line(&mut base_number).unwrap();

        match base_number.trim().parse::<u32>() {
            Ok(value) => break value,
            Err(_) => {
                println!("\nInvalid input, try again...");
                continue;
            }
        };
    };

    println!(
        "\nF({}): {}",
        base_number,
        utils::get_fibonacci(base_number)
    );
}
