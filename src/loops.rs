pub fn run() {
    let mut count = 0;

    // infinite loop
    print!("Infinite Loop: ");
    loop {
        count += 1;
        print!("{} ", count);
        if count == 20 {
            break;
        }
    }
    println!("\n");

    count = 0;
    // while
    print!("While Loop: ");
    while count <= 100 {
        let div_by_3: bool = count % 3 == 0;
        let div_by_5: bool = count % 5 == 0;
        if div_by_3 && div_by_5 {
            print!("FizzBuzz ");
        } else if div_by_3 && div_by_5 {
            print!("Fizz ");
        } else if div_by_5 {
            print!("Buzz ");
        } else {
            print!("{} ", count);
        }
        count += 1;
    }
    println!("\n");

    // for in range
    print!("For In Range Loop: ");
    for x in 0..100 {
        if x % 10 == 0 {
            print!("yey, {} ", x);
        }
    }
    println!("\n");

}
