enum Size {
    BIG,
    SMOL,
}

struct Result {
    size: Option<Size>,
    value: i32,
}

impl Result {
    fn print(self: &Self) {
        let comparison_result = match self.size.as_ref() {
            Some(value) => match value {
                Size::BIG => "big",
                Size::SMOL => "smol",
            },
            None => {
                panic!("Before printing you need to evaluate the result by using the method \"compare\"");
            }
        };

        println!("it's... {comparison_result}");
    }
    fn compare(&mut self) {
        let size = match self.value > 100 {
            true => Some(Size::BIG),
            false => Some(Size::SMOL),
        };

        self.size = size
    }
}

pub fn example() {
    let mut result = Result {
        size: None,
        value: 101,
    };

    result.compare();
    result.print();
}
