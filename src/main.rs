use std::{env, fs::read_to_string, path::Path, process::exit};

use rbfi::interpreter::std_bf::*;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() <= 1 {
        println!("usage: rbfi [filename]\n\npositional arguments:\n\nfilename\tfile that will be interpreted");
        exit(0);
    }

    let mut main_cli: StandardBrainfuck = StandardBrainfuck::new(
        read_to_string(Path::new(&argv[1]))
            .expect("Couldn't read file's content")
            .chars()
            .filter(|x| ['>', '<', '[', ']', '.', ',', '+', '-'].contains(&x))
            .collect(),
    );

    main_cli.run_full_stack();
}
