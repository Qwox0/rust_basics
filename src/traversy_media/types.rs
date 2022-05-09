/* Primative types
Int: u8, i8, u16, ..., i128
-> sign(u,i) bits
Float: f32, f64
bool, char
*/

#[allow(dead_code)]
pub fn run() {
    let x = 1;  // by default: i32
    let y = 2.5;  // by default: f64

    // explicit type
    let z: i64 = 245234973;

    // find max size
    println!("max i32: {}", std::i32::MAX );
    println!("max i64: {}", std::i64::MAX );
    println!("max f64: {}", std::f64::MAX );

    // Boolean
    let is_active = true;
    // from expression
    let is_greater: bool = 10 > 5;

    println!("{:?}", (x, y, z, is_active, is_greater));

    // Char

    let a1 = 'a';
    println!("{}", a1);

    let face = '\u{1F600}';
    println!("{:?}", face);
}
