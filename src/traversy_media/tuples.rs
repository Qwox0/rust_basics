// group of values -> different types
// max 12 elements

#[allow(dead_code)]
pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Mass", 30);

    println!("{} {} is {}", person.0, person.1, person.2);
}
