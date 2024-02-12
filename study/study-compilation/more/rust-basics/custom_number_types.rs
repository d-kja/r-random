fn main() {
    let syntax_sugar = 98_000; // -> 98.000
    let hex_number = 0xfa; // 250
    let binary_number = 0b0010_1011; // 43
    let byte_number = b'A'; // 65
}

/**
 * HOW DOES BINARY WORK
 * 
 * REF: 0b0010_1011
 * 
 * 01. first two chars are there to indicate that it's a binary (0b), and the actual number comes after that
 * 
 * ==> 0b-0010_1011
 * 
 * 02. to count the actual number you go from right to left using (2^n) or (2^index)
 * 
 * byte:   0  0  1  0  1  0  1  1
 * index:  7  6  5  4  3  2  1  0
 * res:  128 64 32 16  8  4  2  1
 * 
 * And now you consider only the active bytes (1 / true) and add them up or multiple them with the byte value and add them up.
 *
 * The total value is -> 32 + 8 + 2 + 1 -> 43
 */

/**
 * HEX/HEXA -> 16
 * 
 * it works the same way as the (BI)NARY but instead of (2^n) it uses (16^n) 
 */


/**
 * THE CONVERSION BETWEEN HEXA AND BINARY WORKS ABOUT THE SAME WAY YOU CALCULATE THE HEXA
 * 
 * Lets say I have the (0x41) hexa which represents b'A'
 * 
 * Use hexa (16^n) and split this: 0x-41
 * 
 * hexa:   4  1
 * index:  1  0
 * res:   16  1
 * 
 * Multiply the result with the hexa and add them up
 * 
 * total -> 64 + 1 -> 65
 */