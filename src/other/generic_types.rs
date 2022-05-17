// Generic Types Test

use std::fmt::Display;

struct Point<T: Display, U: Display> {
    x: T,
    y: U,
}
// Point Structor of two types with Display Trait

impl<T: Display, U: Display> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        return Point { 
            x: x,
            y: y
        };
    }

    fn to_string(&self) -> String {
        // format!() -> String
        return format!("({}|{})", self.x, self.y);
    }
}

pub fn run() {
    let p = Point::new(10, 5.5);
    println!("{}", p.to_string());
}
