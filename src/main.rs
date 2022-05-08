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

// import other files:
// mod print;
// mod vars;
// mod types;
// mod strings;
// mod tuples;
// mod arrays;
// mod vectors;
// mod conditionals;
// mod loops;
// mod functions;
// mod pointer_ref;
// mod structs;
// mod enums;
mod cli;


fn main() {
    // import run() from print.rs
    // print::run();

    // vars::run();
    // types::run();
    // strings::run();
    // tuples::run();
    // arrays::run();
    // vectors::run();
    // conditionals::run();
    // loops::run();
    // functions::run();
    // pointer_ref::run();
    // structs::run();
    // enums::run();
    cli::run();

    // Test
}
