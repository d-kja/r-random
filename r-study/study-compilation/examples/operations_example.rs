fn main() {
    println!("sum {}", sum(5, 5));
    println!("subtraction {}", subtraction(5, 5));
    println!("division {}", division(5, 5));
    println!("multiplication {}\n\n", multiplication(5, 5));

    let mut initial_value: i32 = 0;
    let division_key: i32 = 5;

    loop {
        if initial_value >= 25 {
            break;
        }

        initial_value += 1;
        let current_index: i32 = initial_value;
        let values_tuple: (i32, i32) = (remainder(initial_value, division_key), current_index);

        println!(
            "- current_index: {:?}\n- result: {:?}\n\n",
            values_tuple.1, values_tuple.0
        );
    }
}

fn sum(value_01: i32, value_02: i32) -> i32 {
    return value_01 + value_02;
}

fn subtraction(value_01: i32, value_02: i32) -> i32 {
    value_01 - value_02
}

fn division(value_01: i32, value_02: i32) -> i32 {
    value_01 / value_02
}

fn multiplication(value_01: i32, value_02: i32) -> i32 {
    value_01 * value_02
}

fn remainder(value_01: i32, value_02: i32) -> i32 {
    value_01 % value_02
}
