pub fn run() {
    greeting("Hello", "Tom");

    // bind function values to variable
    let get_sum = add(4, 6);
    println!("Sum: {}", get_sum);

    // closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("closure sum: {} with n3 = {} of current scope", add_nums(3, 3), n3);
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2  // no ; == return
    // return n1 + n2  // Alternative
    // return n1 + n2;  // Alternative
}
