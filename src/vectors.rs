// vectors: resizable arrays

use std::mem;

pub fn run() {
    // let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];  // ref: array
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", numbers);
    println!("first Element: {}", numbers[0]);

    // change
    numbers[2] = 20;

    println!("{:?}", numbers);

    // length
    println!("vector length: {}", numbers.len());

    // stack allocated memory
    println!("this vector occupies {} bytes with {} bytes per element", mem::size_of_val(&numbers), mem::size_of_val(&numbers[0]));

    // array slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);


    // ^^^ same as array ^^^
    // new:

    // add element
    numbers.push(5);
    numbers.push(6);

    // pop
    println!("before: {:?}", numbers);
    println!("pop: {:?}; after: {:?}", numbers.pop(), numbers);

    // loop
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop & mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Vectir after change: {:?}", numbers);



}
