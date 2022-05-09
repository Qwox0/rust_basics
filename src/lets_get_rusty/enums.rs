// Enums: enumerate(aufzählen) different variants

// ------------------
// IpAddress
#[allow(unused)]
enum IpAddressKind {
    IPv4(u8, u8, u8, u8),
    IPv6(String),
}
// Data inside enum variants:
// VariantName(Data)

#[allow(unused)]
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

// ------------------
// Message
#[allow(unused)]
enum Message {
    Quit,                       // -> no Data
    Move { x: i32, y: i32 },    // -> ananymous struct
    Write(String),              // -> String
    ChangeColor(i32, i32, i32), // -> 3 int
}

#[allow(unused)]
impl Message {
    fn some_function() {
        println!("Hello World");
    }
}


// ------------------
// Coin Enum
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}


// =====================
// functions
fn value_in_cents(coin: Coin) -> u64 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


// =====================
// main
pub fn run() {
    // No Data in Enum
    // let localhost = IpAddress {
    //     kind: IpAddressKind::IPv4,
    //     address: String::from("127.0.0.1"),
    // };

    // Enum with Data
    #[allow(unused_variables)]
    let localhost = IpAddressKind::IPv4(127, 0, 0, 1);


    // Option Enum
    // Rust: no null values -> optional Values
    #[derive(Debug)]  // autoimplement Debug Trait
    enum Option2<T> {
        Some(T),
        None,
    } // included by default

    // Examples:
    let some_number: Option2<i32> = Option2::Some(5);
    let some_str = Option2::Some("string");
    let absend_number: Option2<i32> = Option2::None;

    println!("{:?} {:?} {:?}", some_number, some_str, absend_number);

    // Example 2:
    let x: i8 = 5;
    let y: Option<i8> = Some(2);
    // let sum = x + y;  // Error: cannot add i8 type to Option<i8> type
    // -> handle all cases:
    let sum = x + y.unwrap_or(0);

    
}
