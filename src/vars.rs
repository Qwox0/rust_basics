// immutable by default

pub fn run() {
    let name = "Jan";
    let mut age = 18;

    println!("{}", age);
    age = 19; // without mut -> error: age immutable

    println!("My name is {} and i am {}", name, age);

    // Define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let ( a, b ) = ( 1, 2 );
    println!("{} {}", a, b);
}
