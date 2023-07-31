use std::io::{stdin, stdout, Write};

pub fn interpret_bf_str(main_stack: Vec<char>) {
    let mut memory: [u8; 3000] = [0x00; 3000];
    let (mut pointer, mut instruction) = (0, 0);

    while instruction != main_stack.len() {
        match main_stack[instruction] {
            '+' => {
                memory[pointer] = memory[pointer].wrapping_add(1);
            }
            '-' => {
                memory[pointer] = memory[pointer].wrapping_sub(1);
            }
            '>' => {
                pointer += 1;
            }
            '<' => {
                pointer -= 1;
            }
            '.' => {
                print!("{}", memory[pointer] as char);
                stdout().flush().unwrap();
            }
            ',' => {
                let mut input: String = String::new();
                stdin()
                    .read_line(&mut input)
                    .ok()
                    .expect("Failed to read line");
                memory[pointer] = input.bytes().next().expect("no byte read");
            }
            '[' => {
                if memory[pointer] == 0 {
                    instruction = matching_bracket(&main_stack, instruction).expect(
                        "Matching bracket could not be found at instruction number {instruction}",
                    );
                }
            }
            ']' => {
                if memory[pointer] != 0 {
                    instruction = matching_bracket_reversed(&main_stack, instruction).expect(
                        "Matching bracket could not be found at instruction number {instruction}",
                    );
                }
            }
            _ => {}
        }
        instruction += 1;
    }
}

fn matching_bracket_reversed(env: &Vec<char>, offset: usize) -> Option<usize> {
    let mut balance = 0;
    let iterator = (0..(offset + 1)).rev();
    for c in iterator {
        match env[c] {
            '[' => balance += 1,
            ']' => balance -= 1,
            _ => {}
        }
        if balance == 0 {
            return Some(c);
        }
    }
    return None;
}

fn matching_bracket(env: &Vec<char>, offset: usize) -> Option<usize> {
    let mut balance = 0;
    let iterator = offset..(env.len());
    for c in iterator {
        match env[c] {
            '[' => balance += 1,
            ']' => balance -= 1,
            _ => {}
        }
        if balance == 0 {
            return Some(c);
        }
    }
    return None;
}
