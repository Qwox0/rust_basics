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
    Write(String),              // -> 1 String Tuple
    ChangeColor(i32, i32, i32), // -> 3 int Tuple
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
    Silver,
    Gold,
    Custom { value: u64 }, // Custom -> value struct
}

fn get_coin_value(coin: &Coin) -> u64 {
    // coin: borrowed Coin -> *coin: real Coin
    match *coin {
        Coin::Silver => 10,
        Coin::Gold => {
            println!("Gold!");
            1000
        }
        Coin::Custom { value: val } => val,
        // Custom Variant with value val (wildcard: _)
        // -> return val
    }
}

// Alternative: Method
impl Coin {
    fn value(&self) -> u64 {
        match *self {
            // Self == Coin
            Self::Silver => 10,
            Self::Gold => {
                println!("Gold!");
                1000
            }
            Self::Custom { value: val } => val,
        }
    }
}

// =====================
// main
#[allow(unused)]
pub fn run() {
    // --- No Data in Enum
    // let localhost = IpAddress {
    //     kind: IpAddressKind::IPv4,
    //     address: String::from("127.0.0.1"),
    // };
    // --- Enum with Data
    #[allow(unused_variables)]
    let localhost = IpAddressKind::IPv4(127, 0, 0, 1);

    // --- Coins
    let s_coin = Coin::Silver;
    let g_coin = Coin::Gold;
    let my_coin = Coin::Custom { value: 333 };
    println!(
        "silver: {}; gold: {}; my_coin: {}",
        get_coin_value(&s_coin),
        g_coin.value(),
        get_coin_value(&my_coin)
    );
    // Note: g_coin.value() == &g_coin.value()

    // Option Enum
    // Rust: no null values -> optional Values
    #[derive(Debug)] // autoimplement Debug Trait
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
    let _sum = x + y.unwrap_or(0);
    // Option.unwrap(): get value of Some
    //      if None: use unwrap_or() default value
    // _var: unused variable
}
