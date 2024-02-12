enum Example {
    One(u32),
    Two(String),
}

fn main() {
    let example = Example::One(10);
}

fn unwrap_enum_value(example: Example) -> u32 {
    let unwrapped_value = if let Example::One(value) = example {
        value
    } else if let Example::Two(value) = &example {
        value.parse::<u32>().unwrap()
    } else {
        panic!("yeet");
    };

    let unwrapped_value = match example {
        Example::One(value) => value,
        Example::Two(value) => value.parse::<u32>().unwrap(),
    };

    unwrapped_value
}
