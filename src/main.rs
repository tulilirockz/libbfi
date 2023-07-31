use std::{env, fs::read_to_string, path::Path, process::exit};
mod bf_interpreter;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() <= 1 {
        println!("usage: rbfi [filename]\n\npositional arguments:\n\nfilename\tfile that will be interpreted");
        exit(0);
    }

    let bfinstructions: Vec<char> = read_to_string(Path::new(&argv[1]))
        .expect("Couldn't read file's content")
        .chars()
        .filter(|x| ['>', '<', '[', ']', '.', ',', '+', '-'].contains(&x))
        .collect();

    bf_interpreter::interpret_bf_str(bfinstructions);
}
