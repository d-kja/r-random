enum Light {
    Bright,
    Dull,
}

impl Light {
    fn display(light: &Light) {
        match light {
            Light::Bright => println!("It's bright!"),
            Light::Dull => println!("It's dull..."),
        }
    }
}

fn main() {
    let light = Light::Dull;

    Light::display(&light);
    Light::display(&light);
}
