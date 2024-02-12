// https://www.youtube.com/watch?v=rDoqT-a6UFg&t=581s

fn main() {
    ref_of_ref();
}

fn using_copy() {
    let initial_owner = 10;

    let _copied_ref_01 = initial_owner;

    /**
     * The reason as to why the copied_ref didn't take ownership
     * is because i32 is a type that implements the trait Copy
     */
    dbg!(initial_owner);
}

fn moving_ref() {
    let initial_owner = String::from("A simple example");

    let _copied_ref_01 = initial_owner; // string goes in da HEAP, therefore it moves the value instead of copying it, for performance improvements

    /**
     * The reason as to why the copied_ref took ownership is because
     * String is a type that doesn't implements the trait Copy
     */
    dbg!(initial_owner);
}

fn using_mut(mut value: String) -> String {
    value.push_str("-example");
    value
}

fn using_mut_ref_to_mutate() {
    let mut initial_value = String::new();
    let mut_ref = &mut initial_value;

    *mut_ref = String::from("example");
}

fn ref_of_ref() {
    let mut initial_value: i32 = 0;
    let mut copy_ref = &mut initial_value;
    let second_copy_ref = &mut copy_ref;

    // when we are mutating a value by using something like push_str,
    // rust automatically dereferences the variable to reach the original
    // pointer

    // (*variable).push_str("aaaaa!");

    // lets say I want to react the initial value pointer by using the second copy
    // I'd have to derefence manually and twice at that (because the type is &&...)
    **second_copy_ref = 1;

    println!("{:p}", second_copy_ref);
    println!("{:p}", &copy_ref);
    println!("{:p}", copy_ref);
    println!("{:p}", &initial_value);
    println!("{:?}", initial_value);
}
