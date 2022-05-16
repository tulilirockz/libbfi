use std::env;
use std::path::Path;
use std::process::exit;
use std::fs::read_to_string;
mod bf_interpreter;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    if argc <= 1 {
        println!("usage: bbfi [filename]\n\npositional arguments:\n	filename	file that will be interpreted! (must have B.F. code)");
        exit(0);
    } else {
        let filepath: &Path = Path::new(&argv[1]);

        if !filepath.exists() {
            println!("File '{}' does not exist or is not a valid file.", &argv[1]);
            exit(1)
        }

        let everything: String = read_to_string(filepath)
                                 .expect("Couldn't read file's content");

        let bfinstructions: Vec<char> = everything.chars()
                                                .filter(|x| {['>', '<', '[', ']', '.', ',', '+', '-'].contains(&x)})
                                                .collect();

        bf_interpreter::interpret_bf_str(bfinstructions);
    }
}
