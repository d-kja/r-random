fn main() {
    display_result(sum(25, 43));
}

fn display_result(result: i32) {
    // Display result to the user
    println!("The result of the sum was: {result:?}");
}

fn sum(value_01: i32, value_02: i32) -> i32 {
    value_01 + value_02
}
