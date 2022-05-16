use std::fmt::Display;
#[allow(unused)]
use std::num::ParseFloatError;

// 1.
fn string_to_number(str: String) -> Option<f64> {
    // match str.parse::<f64>() {
    //     Ok(num) => return Option::Some(num),
    //     Err(_) => {
    //         println!("invalid Number String");
    //         Option::None
    //     }
    // } // -> Option<f64>

    str.parse::<f64>().ok() // -> Option<f64>

    // str.parse::<f64>() // -> Result<f64, ParseFloatError>
}

// 2.
fn check_var<T>(var: T, fail: bool) -> Result<T, String> {
    let err = "Error!".to_string();
    match fail {
        false => Result::Ok(var),
        true => Result::Err(err),
    }
}

// impl<T, E> Result<T, E> {} // -> not allowed: Result defined in other crate
fn result_to_string<T: Display, E: Display>(r: Result<T, E>) -> String {
    match r {
        Ok(val) => format!("{}", val),
        Err(err) => format!("{}", err),
    }
}

// Main
#[allow(unused)]
pub fn run() {
    // 1.
    println!("----------");
    let num_strings = ["3987", "3abc", "3.987"];

    for str in num_strings {
        println!("{:?}", string_to_number(str.to_string()));
    }

    // 2.
    println!("----------");
    println!(
        "Result 3 to String: {}",
        result_to_string(check_var(3, false))
    );
    println!(
        "Result error to String: {}",
        result_to_string(check_var(3, true))
    );
}
