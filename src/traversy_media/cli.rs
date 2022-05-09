// cli: Command Line interface

// cargo run <params>

#[allow(dead_code)]
pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();
    let name = "World";
    let status = "99%";

    println!("Args: {:?}", args);  // -> ["/path/to/executable", "arg1", ...]
    println!("Command: {}", command);

    if command == "hello" {
        println!("Hello {}!", name);
    } else if command == "status" {
        println!("status is: {}", status );
    } else {
        println!("{} is not a valid command", command);
    }
}
