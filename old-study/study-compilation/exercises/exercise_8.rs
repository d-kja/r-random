use core::fmt;

#[derive(Debug)]
enum Flavors {
    Bitter,
    Sour,
    Salty,
    Sweet,
}

impl fmt::Display for Flavors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //  match *self
        match self {
            Flavors::Sour => write!(f, "Sour"),
            Flavors::Bitter => write!(f, "{:?}", self),
            Flavors::Salty => write!(f, "{:?}", self::Flavors::Salty),
            Flavors::Sweet => write!(f, "{:?}", self::Flavors::Sweet),
        }
    }
}

struct Drink {
    flavour: Flavors,
    ounces: f32,
}

impl Drink {
    fn print(&self) {
        let formatted_str = format!(
            "Drink information:\n - Flavour: {}\n - Ounces: {}",
            self.flavour, self.ounces
        );

        // let formatted_str = format!(
        //     "Drink information:\n - Flavour: {}\n - Ounces: {}",
        //     match self.flavour {
        //         Flavors::Bitter => format!("Bitter"),
        //         Flavors::Sour => format!("Sour"),
        //         Flavors::Salty => format!("Salty"),
        //         Flavors::Sweet => format!("Sweet"),
        //     },
        //     self.ounces
        // );

        println!("{formatted_str}");
    }
}

pub fn example() {
    let my_drink_of_choice = Drink {
        flavour: Flavors::Sour,
        ounces: 150.50,
    };

    my_drink_of_choice.print();
}
