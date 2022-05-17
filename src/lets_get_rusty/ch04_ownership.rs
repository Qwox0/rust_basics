/*  Ownership:
    guarantee Memory Safety without garbage collector

    Memory:
    Stack
        fixed size during runtime
        containes stackframes for every function
            local function variables (live as long as the stackframe)
            size calculated at compile time
            function started -> push to stack
            function return -> pop from stack (local vars dropped)
    Heap
        grow, shrink at runtime
        less organized (no push, pop order)
        can contain dynamically sized vars

    more Examples:
    fn b -> let x: String
        push b to stack -> allocate mem for String x in Heap -> pointer to x stored in stack
        push(Stack) faster than allocate(Heap)

    other:
    &str: String Literal
        stored in Binary -> fixed size
*/

#[allow(unused)]
pub fn run() {
    /*
        !!! Rules !!!
        1. Each value in Rust has a variable that's called its owner.
        2. Only one owner at a time.
        ∀val ∃!var ( var.owns(val))
        3. When the owner goes out of scope, the value will be dropped.
    */

    // Example:
    // s not valid
    {
        let mut s = "Hello"; // s is declared
        println!("{} World", s); // do stuff with s
        s = "Hi";
        println!("{} World", s);
    } // s out of scope -> s not valid

    let x = 5;
    let y = x;
    // y: copy of x
    println!("x: {}; y: {}", x, y);

    let s1 = String::from("Hi");
    let s2 = s1;
    // s2: moved value of s1 -> s1 doesnt exist anymore
    /*
        s1 after deklaration/initialization
        Stack (offset: S_a)
        pointer  | -> points to Heap
        len      | 2
        capacity | 2

        pointer to Heap (offset: H_a)
        index | value
        0     | h
        1     | i

        -------------------------
        s2 after deklaration/initialization
        1. NOT: (exact copy)
        Stack (offset: S_b)
        [copy of S_a]
        pointer to Heap (offset: H_b)
        [copy of H_a]

        2. NOT: (exact shallow copy)
        Stack (offset: S_b)
        [copy of S_a]
        pointer to Heap (offset: H_a)
        [same as s1]
    */
    // println!("s1: {}; s2: {}", s1, s2);  // -> Error
    println!("s1: Error(s1 is moved); s2: {}", s2);

    // create actuall copy
    let s3 = s2.clone();
    println!("s1: Error(s1 is moved); s2: {}; s3: {}", s2, s3);
    /*
        by default: int, float, bool, char, ... are copied (they implement Copy Trait)
        String, Vec, ... are moves
    */

    // ============================
    // function takes ownership
    let s = String::from("Hello World");
    takes_ownership(s);
    // println!("{}", s); // Error: s is moved into param of takes_ownership()

    // function copies primitive variable
    let x = 5;
    makes_copy(x);
    println!("main() still owns {}", x);

    // main gets ownership
    let s1 = gives_ownership();
    println!("main() gets s1: \"{}\"", s1);

    // borrow String
    let mut s1 = String::from("Hello");
    let str_length = borrows_string(&mut s1);
    println!("The length of \"{}\" is {}", s1, str_length);
}

fn takes_ownership(some_string: String) {
    println!("I own \"{}\"", some_string);
}

fn makes_copy(some_num: isize) {
    println!("I copied \"{}\"", some_num);
}

fn gives_ownership() -> String {
    let s = String::from("I am from gives_ownership()");
    return s;
}

fn borrows_string(borrowed_str: &mut String) -> usize {
    borrowed_str.push_str(", world");
    borrowed_str.len()
}
