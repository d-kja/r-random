use core::fmt;
use std::io;

enum Menu {
    Home,
    Editing,
    Exiting,
}

impl fmt::Display for Menu {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Menu::Home => write!(formatter, "home"),
            Menu::Editing => write!(formatter, "editing"),
            Menu::Exiting => write!(formatter, "exiting"),
        }
    }
}

impl Menu {
    fn get_menu(choice: &str) -> Result<Self, String> {
        match choice {
            "home" => Ok(Self::Home),
            "editing" => Ok(Self::Editing),
            "exiting" => Ok(Self::Exiting),
            _ => Err("Invalid option".to_owned()),
        }
    }

    fn print(choice: &str) -> Result<(), String> {
        // It's going to return a Err(...) if it tries to unwrap and an Error is the result
        let menu = Self::get_menu(choice)?;

        println!("Result: {menu}");

        Ok(())
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;

    let choice = user_input.trim_end();
    let result: Result<_, String> = Menu::print(choice);

    if let Err(value) = result {
        println!("{value:?}");
    }

    Ok(())
}
