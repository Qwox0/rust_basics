// cli: Command Line interface

// cargo run <params>

#[allow(dead_code)]
pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let arg_count = args.len();
    // Remember: args[0] = "/path/to/executable" => args.len() >= 1
    println!("number of args: {}", arg_count);

    if arg_count > 1 {
        call_command(args);
    }
}

fn call_command(args: Vec<String>) {
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
