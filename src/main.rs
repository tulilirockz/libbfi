use std::fs::read_to_string;

use clap::Parser;
use rbfi::interpreter::{ook::Ook, std_bf::*};

#[derive(Parser)]
struct MainCli {
    type_of_program: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = MainCli::parse();

    let usage_data: String = read_to_string(args.path).expect("Couldn't read file's content");

    match args.type_of_program.as_str() {
        "ook" => Ook::new(usage_data)
            .filter_characters()
            .expect("Failed to parse characters")
            .run_full_stack(),
        "std" => StandardBrainfuck::new(usage_data)
            .filter_characters()
            .expect("Failed parsing characters")
            .run_full_stack(),
        _ => panic!("Failed to find a valid dialect!"),
    }
}
