// --- How to use Rust manually
// compile: rustc main.rs
// execute: ./main

// --- How to use Cargo
// new project: cargo new projectName
// use current folder: cargo init
// -> creates .gitignore -> ./target; Cargo.toml; ./src

// compile: cargo run                   : build & run
// -> ./target: has compiler files
// ./target/debug/projectName: Executable!
// Alternatives: cargo build            : just build
// Alternatives: cargo build --release  : build for release

// import other files/dirs:
mod lets_get_rusty;
mod other;
mod traversy_media;

fn main() {
    // import run() from traversy_media/cli.rs
    // traversy_media::cli::run();

    lets_get_rusty::ch02_guessing_game::run();
}
