// Standard Input/Output

use std::io; // stdio module

// See Cargo.toml
use colored::*;
use rand::Rng;

use std::cmp::Ordering; // result enum of a number comparison

#[allow(unused)]
pub fn run() {
    let secret_number = rand::thread_rng().gen_range(1..101); // 101 exclusive
                                                              // println!("\nSecret number: {}", secret_number);

    loop {
        println!("\nGuess the number (1 to 100)");
        let mut guess: String = String::new();
        // input guess
        io::stdin()
            .read_line(&mut guess) // -> Result enum: Ok(T) or Err(E)
            .expect("Failed to read line!"); // if Ok -> return Ok(T); if Err -> return "msg"

        // convert to int
        // Note: Shadowing: same var name
        // let guess: i32 = guess
        //     .trim()  // remove whitespaces at beginning/end
        //     .parse()  // -> Result enum: i32 if successful else PANIC!
        //     .expect("Please type a number!");  // if Result enum = Err
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // _ (wildcard) for Error -> next loop cycle -> no Panic
                println!("Please enter a number");
                continue;
            }
        };

        println!("Your Guess: {}", guess);
        // compare guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                break;
            }
        };
    } // end loop
}
