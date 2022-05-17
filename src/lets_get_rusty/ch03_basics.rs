#[allow(unused)]
pub fn run() {
    println!("------ Vars ------");
    let mut x = 5;
    println!("value of x: {}", x);
    x = 6;
    println!("value of x: {}", x);

    const BIG_VALUE: i32 = 1_000_000;
    // Differences to "let big_value":
    // mut-keyword doesnt work
    // no auto type detection (": i32" required)
    // value must be clear at compile time (no function returns)
    println!("{}", BIG_VALUE);

    // Shadowing
    // overwrite variable; without "mut"
    // or change type
    println!("------ Shadowing ------");
    let str = "Hello";
    println!("value of str: {}", str);
    let str = "Hello World";
    println!("value of str: {}", str);
    let str = 5;
    println!("value of str: {}", str);

    // Tuples
    println!("------ Tuples ------");
    let tup = ("Hello World", 100_000);
    // destructuring
    let (str, x) = tup;
    println!("tup: {:?} = ( {}, {} )", tup, str, x);
    // dot notation
    println!("first Tup value{}", tup.0);

    // Arrays
    // fixed length, fixed type
    println!("------ Arrays ------");
    let num_arr: [i32; 3];
    num_arr = [3, 10, 999_999];
    println!("big: {}", num_arr[2]);

    //
    println!("------------");
    let condition: bool = true;
    println!("condition: {}", condition);
    let num = if condition { 3 } else { 6 };
    println!("if condition {{ 3 }} else {{ 6 }}: {}", num);

    // loops
    println!("------ loops ------");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("INFINITE loop!");

        if counter == 10 {
            break counter;
            // returns counter
        }
    };
    println!("after loop | counter: {}; result: {}", counter, result);

    // while
    println!("------ While ------");
    let mut num = 3;
    while num > 0 {
        println!("{}", num);
        num -= 1;
    }

    // For
    println!("------ For ------");
    let arr = [10, 20, 30, 40, 51];
    for element in arr.iter() {
        println!("{}", element);
    }

    // For in range
    println!("------ For in range ------");
    for num in 1..=3 {
        println!("{}", num);
    }
}
