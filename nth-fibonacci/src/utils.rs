// Fn = Fn-1 + Fn-2
// F0 = 0 and F1 = 1.
pub fn get_fibonacci(number: u32) -> u32 {
    // println!("- {}", number);

    // 1 -> 1 | 0 -> 0
    if number < 2 {
        return number;
    }

    get_fibonacci(number - 1) + get_fibonacci(number - 2)
}
