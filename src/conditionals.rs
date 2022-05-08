// conditionals -> check conditions

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person: bool = true;

    // if
    if age >= 21 && check_id || knows_person {
        println!("hi");
    } else if age < 21 && check_id {
        println!("Too young!");
    } else {
        println!("check id");
    }

    // short if
    let is_of_age: bool = if age >= 21 {true} else {false};
    let is_of_age2: bool = age >= 21;

    println!("Is of age?: {:?}", is_of_age);
    println!("Is of age2?: {:?}", is_of_age2);
}
