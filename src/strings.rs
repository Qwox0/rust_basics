// Privitive string: immutable, fixed-legth
// String: growable, heap-allocated data structure

pub fn run() {
    let hello = "Hello"; // privitive string
    let mut hello2 = String::from("Hello");

    // length
    println!("Length: {}, {}", hello.len(), hello2.len() );

    // add chars: only for String
    hello2.push('W');  // only for char
    println!("{}", hello2);

    hello2.push_str("orld");
    println!("{}", hello2);

    // String Methods
    println!("Capacity: {}", hello2.capacity());
    println!("is empty?: {}", hello2.is_empty());
    println!("Contains 'World': {}", hello2.contains("World") );
    println!("Replace: {}", hello2.replace("World", " World!") );  // only for print
    hello2 = hello2.replace("World", " World!");  // change var
    println!("test: {}", hello2);
    
    // Loop through string by whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing (behaupte Gleichheit)
    assert_eq!(2, s.len());  // pass
    // assert_eq!(3, s.len()); // -> Error: thread 'main' panicked at 'assertion failed

    assert_eq!(10, s.capacity());

    println!("all assertions passed!");
}
