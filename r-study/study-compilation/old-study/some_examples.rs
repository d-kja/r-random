use std::fmt;

fn main() {
    _references();
}

fn _references() {
    let mut mutable_variable: i32 = 10;
    let unmutable_variable: &mut i32 = &mut mutable_variable;

    *unmutable_variable = 333;

    print!("unmut: {unmutable_variable}\n");

    print!("{}", unmutable_variable.count_ones());
}

fn _arrays_and_tuples_example() {
    println!(r#"<a href="example.html">link example</a>"#); // disabling the need for escapes on quotes by using r#"..."#

    // [i8; 5] => [type; fixed size] // [10; 5] => fill 5 slots with 10
    let mut array_example: [i8; 5] = [10; 5];

    // reasign slot 2 with 0
    array_example[2] = 0;

    // format array in a bunch of lines
    println!("\narray: {:#?}", array_example);

    // print it in a single line
    println!("\narray: {array_example:?}");

    let tuple_example: (&str, bool) = ("example", true);

    println!(
        "\ntuple: {:?}\n - slot 01: {}\n - slot 02: {}",
        tuple_example, tuple_example.0, tuple_example.1
    );
}

fn _integer_example() {
    let unsigned_interger: u8; // i8 is a signed integer, u8 siginifies that the integer isn't assigned to anything yet.

    // "Mutating"/assigning it after declaring
    unsigned_interger = 3;
    print!("{unsigned_interger} \n");

    // working with floating numbers
    let floating_number: f32 = 3.12;
    println!("{floating_number}")
}

fn _fmt_example() {
    let example: String = fmt::format(format_args!("{:07}", 123));
    print!("{}", example);
}

fn _basic_example() {
    let const_example: &str = "my first hello world";
    let example_result: i32 = _random_example(false);
    // let example_overflow: i8 = 222; i8 has a range of -127-126

    print!("Executing my first custom function, returning: {example_result} \n");
    println!("Hey! you can call me hlyd and this is {}", const_example);

    let mut mutable_example: i32 = _random_example(true);

    while mutable_example != 1 {
        if mutable_example % 2 == 0 {
            mutable_example = mutable_example / 2;
        } else {
            mutable_example = 3 * mutable_example + 1;
        }

        print!("- {mutable_example} \n");
    }
}

fn _random_example(example: bool) -> i32 {
    if example {
        return 100;
    }

    for i in [1, 2, 3] {
        println!("-> {i}");
    }

    4
}
