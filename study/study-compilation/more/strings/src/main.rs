fn main() {
    println!("Hello, world!");
}

fn strings_vs_slice() {
    // strings are volatile and can change at any time
    // for that reason they go into the heap memory
    let example_string = String::new();
    // /\ this creates a Heap string value, it's more expensive

    // In the other hand, slice is a more static version of string, a less expensive one.
    // It still is store in the heap, but only as a readonly value.
    // IIRC, rust allocates those values at the beginning of the program to be used during runtime
    // simplifying the process of using such a volatile type and making it less expensive to the program in general
    let example_slice = "I'm a slice";
    let example_slice_from_string = &example_slice[2..=4];

    // Heap Strings can be mutated as long as they're mutable variables
    // slices cannot be mutated, but they can be replaced for new ones
    let mut example_string = String::from("Hello");
    example_string.push_str(" World!");

    let mut example_slice = "Hello";
    example_slice = "Hello World!";

    /**
     *  Rust doesn't let you copy expensive types by default. Strings are one of those.
     *
     */
    // Usually with other languages if you do the following it copies the value:
    let example = 0;
    let example_copy = example;

    // The way that rust handles that is by using traits,
    // and Assigned and unsigned integers have the Copy trait by default

    // But strings don't, and the same happens for more expensive ones like structs and enums.
    // So the following doesn't work the same way as it would in other languages
    let example = String::new();
    let example_copy = example;

    // The truth is that it doesn't copy, it steals the pointer reference.
    // To copy that expensive value you have to either clone
    // or implement the copy trait for that type.
    let example = String::new();
    let example_actual_copy = example.clone();
}
