#[allow(dead_code)]
pub fn run() {
    // print to console
    println!("Hello from from print.rs");

    // Basic Formating
    // println!(1); // -> Error 1 not string
    println!("Number: {}", 1);

    println!("e: {}; 1: {}", "e", 1);

    // Positional Arguments
    println!("1: {0}, 2: {1}, 1: {0}", 1, 2);

    // Named Arguments
    println!("{name} likes {activity}", name = "J", activity = "sport");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, octal: {:o}", 10, 10, 10);

    // Placeholder for Debug trait
    println!("{:?}", (12, true, "hello"));

    // basic Math
    println!("10 + 10 = {}", 10 + 10);
}
