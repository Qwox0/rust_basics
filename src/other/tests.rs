// enum Numbers {
//     I32 = 0
// }

#[allow(unused)]
pub fn run() {
    println!("{:?}", add(3, 5));
}

#[allow(unused)]
pub fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

#[cfg(test)] // test Block: Only compiled on 'cargo test'
mod tests {
    use super::*; // use everything in test.rs
    #[test] // one test Block
    fn it_works() {
        assert_eq!(add(1, 1), 2);
    }
}
