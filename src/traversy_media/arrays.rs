// array: fixed, same type

use std::mem;

#[allow(dead_code)]
pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];  // must be 5 integer init values

    println!("{:?}", numbers);
    println!("first Element: {}", numbers[0]);

    // change
    numbers[2] = 20;

    println!("{:?}", numbers);

    // length
    println!("array length: {}", numbers.len());

    // stack allocated memory
    println!("this array occupies {} bytes with {} bytes per element", mem::size_of_val(&numbers), mem::size_of_val(&numbers[0]));

    // array slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
