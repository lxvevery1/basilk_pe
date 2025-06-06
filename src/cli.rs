use std::{env, process::exit};

pub struct Cli;

impl Cli {
    pub fn read() {
        // If you use `cargo run main.rs`, skip must be 2
        let mut args = env::args().skip(1);

        if let Some(arg) = args.next() {
            if arg == "--version" {
                print!(env!("CARGO_PKG_VERSION"));
                exit(0)
            }
        }
    }
}
