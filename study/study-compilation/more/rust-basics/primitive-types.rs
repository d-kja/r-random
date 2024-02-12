struct Primitives<'static a> {
    slice: &'a str, // -> Borrowed slice
    string: String, // -> Heap string

    number_8bits: u8, // -> unsigned 8 bits integer
    number_16bits: u16, // -> unsigned 16 bits integer
    number_32bits: u32, // -> unsigned 32 bits integer
    number_64bits: u64, // -> unsigned 64 bits integer

    float_32bits: f32, // -> float of 32 bits
    float_64bits: f64, // -> float of 64 bits
}

/**
 * OBSERVATIONS
 * 
 * UNSIGNED NUMBERS -> can only hold positive values and the math for how much it can hold varies from the signed ones
 * SIGNED NUMBERS -> can hold both positive and negative values, however the range for the positives shrink a bit from the unsined ones
 * 
 * UNSIGNED -> 2 ^ N - 1
 * SIGNED -> -2 ^ (N - 1) ... 2 ^ (N - 1) - 1
 */