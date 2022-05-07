// Interprets the whole fucking program

// Memory pointer -> Mem value (gets manipulated throught the program)
// Tape pointer -> Instruction (basically works like the for_loop thing)
use std::collections::VecDeque;
use std::io::stdin;
use std::io::{Write};

pub fn matching_bracket(env: &Vec<char>, offset: usize, reverse: bool) -> Option<usize> {
    let mut balance = 0;

    if reverse {
        let amongs = (0..(offset + 1)).rev();
        for c in amongs {
            match env[c] {
                '[' => { balance += 1 }
                ']' => { balance -= 1 }
                _ => {}
            }
            if balance == 0 {
                return Some(c);
            }
        }
        return None;
    } else {
        let amongs = offset..(env.len());
        for c in amongs {
            match env[c] {
                '[' => { balance += 1 }
                ']' => { balance -= 1 }
                _ => {}
            }
            if balance == 0 {
                return Some(c);
            }
        }
        return None;
    }
}

pub fn interpret_bf_str(main_stack: Vec<char>) {
    let mut memory_tape: VecDeque<u8> = VecDeque::from([0x00]);
    let mut mem_pointer: usize = 0;
    let mut ins_pointer: usize = 0;

    while ins_pointer != main_stack.len() {
        match main_stack[ins_pointer] {
            '>' => {
                if mem_pointer + 1 >= memory_tape.len() {
                    memory_tape.push_back(0);
                }    
                mem_pointer += 1;
            } 
            '<' => {
                if mem_pointer <= 0 {
                    memory_tape.push_front(0);
                }
                mem_pointer -= 1;
            }
            '+' => {
                memory_tape[mem_pointer] = memory_tape[mem_pointer].wrapping_add(1);
            }
            '-' => {
                memory_tape[mem_pointer] = memory_tape[mem_pointer].wrapping_sub(1);
            }
            '.' => {
                print!("{}", memory_tape[mem_pointer] as char);
                std::io::stdout().flush().unwrap();
            }
            ',' => {
                let mut input: String = String::new();
                let mut _cool_string: usize;
                _cool_string = stdin().read_line(&mut input).ok().expect("Failed to read line");
                memory_tape[mem_pointer] = input.bytes().next().expect("no byte read");
            }
            '[' => {
                if memory_tape[mem_pointer] == 0 {
                    ins_pointer = matching_bracket(&main_stack, ins_pointer, false).expect("Matching bracket could not be found at instruction number {ins_pointer}");
                }
            }
            ']' => {
                if memory_tape[mem_pointer] != 0 {
                    ins_pointer = matching_bracket(&main_stack, ins_pointer, true).expect("Matching bracket could not be found at instruction number {ins_pointer}");
                    continue;
                }
            }
            _ => {}
        }
        ins_pointer += 1;
    }
}