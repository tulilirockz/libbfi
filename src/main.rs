use std::{env, fs::read_to_string, path::Path, process::exit};

use rbfi::interpreter::{ook::Ook, std_bf::*};

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() <= 1 {
        println!("usage: rbfi [filename]\n\npositional arguments:\n\nfilename\tfile that will be interpreted");
        exit(0);
    }

    let mut main_cli = Ook::new(
        read_to_string(Path::new(&argv[1]))
            .expect("Couldn't read file's content")
            .chars()
            .collect(),
    );
    main_cli.filter_characters().run_full_stack();
}
