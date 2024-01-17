use std::fs;

use clap::Parser;

/// CLI to print file Idk lol
#[derive(Parser, Debug)]
struct Cli {
    /// File to read
    #[arg(short)]
    input_file: String,

    /// Convert to single quotes?
    #[arg(short, default_value_t = false)]
    single_quote: bool,

    /// Increment file text
    #[arg(short)]
    content: Option<String>,
}

fn main() {
    let args: Cli = Cli::parse();

    let file: String = match fs::read_to_string(&args.input_file) {
        Ok(value) => value,
        Err(_) => panic!("Invalid path"),
    };

    let mut file: String = match args.single_quote {
        true => file.replace("\"", "'"),
        false => file,
    };

    match args.content {
        Some(value) => {
            file += &format!(" {:}", value);
        }
        None => (),
    }

    let path = args.input_file.clone();

    match fs::write(path, file) {
        Ok(_) => println!("File updated"),
        Err(_) => panic!("Unable to write file"),
    };
}
