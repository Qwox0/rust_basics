// custom data types
// Struct == class in other languages

// traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct ColorTup(u8, u8, u8);


struct Person {
    first_name: String,
    last_name: String
}
impl Person {
    // Constructor
    fn new(first: &str, last: &str) -> Person {
        return Person { first_name: first.to_string(), last_name: last.to_string() };
    }

    // get full name
    fn full_name(&self) -> String {
        // &self: borrows self -> p is still available after p.full_name()
        return format!("{} {}", self.first_name, self.last_name);
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        // self: moves self -> p not available anymore
        return (self.first_name, self.last_name);

        // Alternative: self
        // return (self.first_name.to_string(), self.last_name.to_string());
    }
}

#[allow(dead_code)]
pub fn run() {
    let mut c = Color {red: 255, green: 0, blue: 0};
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = ColorTup(250, 0, 0);
    c2.0 = 255;
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);


    let mut p = Person::new("John", "Doe");
    println!("Person: {} {}", p.first_name, p.last_name);
    println!("full name: {}", p.full_name() );
    p.set_last_name("New");
    println!("full name: {}", p.full_name() );

    println!("name tuple: {:?}", p.to_tuple() );

    // println!("full name: {}", p.full_name() );  // Error: p not available anymore after to_tuple() took ownership
}
