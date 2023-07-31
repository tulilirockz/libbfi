use std::{collections::VecDeque, env, fs::read_to_string, path::Path, process::exit};

use bf_interpreter::{StandardBrainfuck, StdProgram};
mod bf_interpreter;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() <= 1 {
        println!("usage: rbfi [filename]\n\npositional arguments:\n\nfilename\tfile that will be interpreted");
        exit(0);
    }

    let mut main_cli = StandardBrainfuck {
        instruction_stack: read_to_string(Path::new(&argv[1]))
            .expect("Couldn't read file's content")
            .chars()
            .filter(|x| ['>', '<', '[', ']', '.', ',', '+', '-'].contains(&x))
            .collect(),
        memory: VecDeque::from([0x00]),
        ..Default::default()
    };

    main_cli.init_interpretation();
}
