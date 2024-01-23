struct Temperature {
    degrees: f64,
}

impl Temperature {
    fn boiling() -> Temperature {
        Temperature { degrees: 74.0 }
    }

    fn freezing() -> Self {
        Self { degrees: 0.0 }
    }

    fn display(self: &Temperature) {
        println!("{} degrees", self.degrees)
    }
}

fn main() {
    let hot_temperature = Temperature { degrees: 37.0 };
    let boiling = Temperature::boiling();
    let freezing = Temperature::freezing();

    hot_temperature.display();
    boiling.display();
    freezing.display();
}
