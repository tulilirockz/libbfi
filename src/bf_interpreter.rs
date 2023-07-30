use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

pub fn interpret_bf_str(main_stack: Vec<char>) {
    let mut mem_tape: VecDeque<u8> = VecDeque::from([0x00]);
    let mut tape_ptr: usize = 0;
    let mut inst_ptr: usize = 0;

    while inst_ptr != main_stack.len() {
        match main_stack[inst_ptr] {
            '+' => {
                mem_tape[tape_ptr] = mem_tape[tape_ptr].wrapping_add(1);
            }
            '-' => {
                mem_tape[tape_ptr] = mem_tape[tape_ptr].wrapping_sub(1);
            }
            '>' => {
                if tape_ptr + 1 >= mem_tape.len() {
                    mem_tape.push_back(0)
                }
                tape_ptr += 1;
            }
            '<' => {
                if tape_ptr <= 0 {
                    mem_tape.push_front(0);
                }
                tape_ptr -= 1;
            }
            '.' => {
                print!("{}", mem_tape[tape_ptr] as char);
                stdout().flush().unwrap();
            }
            ',' => {
                let mut input: String = String::new();
                stdin()
                    .read_line(&mut input)
                    .ok()
                    .expect("Failed to read line");
                mem_tape[tape_ptr] = input.bytes().next().expect("no byte read");
            }
            '[' => {
                if mem_tape[tape_ptr] == 0 {
                    inst_ptr = matching_bracket(&main_stack, inst_ptr).expect(
                        "Matching bracket could not be found at instruction number {ins_pointer}",
                    );
                }
            }
            ']' => {
                if mem_tape[tape_ptr] != 0 {
                    inst_ptr = matching_bracket_reversed(&main_stack, inst_ptr).expect(
                        "Matching bracket could not be found at instruction number {inst_ptr}",
                    );
                }
            }
            _ => {}
        }
        inst_ptr += 1;
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
