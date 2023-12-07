use owo_colors::OwoColorize;
use std::io;

fn main() {
    let hr: &str = "===========================================";
    println!(
        "{}\n\n       {}\n\n{}\n",
        hr.dimmed(),
        "TEMPERATURE CONVERSION APP".bold(),
        hr.dimmed()
    );

    println!(
        "{}\n\n  {} - CELSIUS TO FAHRENHEIT\n  {} - FAHRENHEIT TO CELSIUS\n",
        "## OPTIONS FOR CONVERSION AVAILABLE ##".dimmed().bold(),
        "[0]".bold().bright_cyan(),
        "[1]".bold().bright_cyan()
    );

    println!("{}\n", "Select an option to continue...".dimmed());

    'outer: loop {
        // USE CLI PIPE TO GET USER INPUT AND PARSE INTO A VALID NUMBER, RECOVERING FROM INVALID INPUTS
        let user_input = loop {
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Unable to read input...");

            match user_input.trim().parse::<u8>() {
                Ok(value) => break value,
                Err(_) => {
                    println!("{}", "Invalid input, try again...".red());
                    continue;
                }
            }
        };

        loop {
            match user_input {
                0 => {
                    println!(
                        "\n{}\n{}\n",
                        "Selected conversion from celsius to fahrenheit...".dimmed(),
                        "Provide the temperature in celsius to convert:"
                            .bold()
                            .bright_yellow()
                    );

                    let mut temperature_input: String = String::new();
                    io::stdin()
                        .read_line(&mut temperature_input)
                        .expect("Unable to read input");

                    let temperature_input = match temperature_input.trim().parse::<f64>() {
                        Ok(value) => value,
                        Err(_) => {
                            println!("{}", "Invalid input, try again...".red());
                            continue;
                        }
                    };

                    println!(
                        "The result was: {} °F",
                        from_celsius_to_fahrenheit(temperature_input).red()
                    );

                    break 'outer;
                }
                1 => {
                    println!(
                        "\n{}\n{}\n",
                        "Selected conversion from fahrenheit to celsius...".dimmed(),
                        "Provide the temperature in fahrenheit to convert:"
                            .bold()
                            .bright_blue()
                    );

                    let mut temperature_input: String = String::new();
                    io::stdin()
                        .read_line(&mut temperature_input)
                        .expect("Unable to read input");

                    let temperature_input = match temperature_input.trim().parse::<f64>() {
                        Ok(value) => value,
                        Err(_) => {
                            println!("{}", "Invalid input, try again...".red());
                            continue;
                        }
                    };

                    println!(
                        "The result was: {} °C",
                        from_fahrenheit_to_celsius(temperature_input).red()
                    );

                    break 'outer;
                }
                _ => {
                    println!("Invalid option, try again...");
                    continue;
                }
            }
        }
    }
}

// (0°C × 9/5) + 32
fn from_celsius_to_fahrenheit(temperature: f64) -> f64 {
    (temperature * 1.8) + 32.0
}

// (32°F − 32) / 5/9 = 0°C
fn from_fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) / 1.8
}
