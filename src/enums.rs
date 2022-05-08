// Enums - has few definite values

// Enum type
enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // action depending on info
    match m {
        Movement::Up => println!("move up"),
        Movement::Down => println!("move down"),
        Movement::Left => println!("move left"),
        Movement::Right => println!("move right"),
    }
    // must contain all possibilities
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
