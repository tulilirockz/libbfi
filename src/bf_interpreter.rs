use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

/// A Standard brainfuck interpreter
///
/// This provides an implementation of the `StdProgram` trait for the Brainfuck
/// language.
/// Uses a Double Queue Vector to represent the memory tape and a Vector to store the
/// Brainfuck instructions.
/// Allows for unlimited memory and memory wrapping
#[derive(Default)]
pub struct StandardBrainfuck {
    pub memory: VecDeque<u8>,
    pub instruction_stack: Vec<char>,
    pub pointer: usize,
    pub instruction: usize,
}

pub trait StdProgram {
    fn matching_bracket(instruction_set: &Vec<char>, offset: usize) -> Option<usize> {
        let mut balance = 0;
        let iterator = offset..(instruction_set.len());
        for c in iterator {
            match &instruction_set[c] {
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
    fn next_instruction(&mut self);
    fn init_interpretation(&mut self);
}

impl StdProgram for StandardBrainfuck {
    fn next_instruction(&mut self) {
        match self.instruction_stack[self.instruction] {
            '+' => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
            }
            '-' => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
            }
            '>' => {
                if self.pointer + 1 >= self.memory.len() {
                    self.memory.push_back(0)
                }
                self.pointer += 1;
            }
            '<' => {
                if self.pointer <= 0 {
                    self.memory.push_front(0);
                }
                self.pointer -= 1;
            }
            '.' => {
                print!("{}", self.memory[self.pointer] as char);
                stdout().flush().unwrap();
            }
            ',' => {
                let mut input: String = String::new();
                stdin()
                    .read_line(&mut input)
                    .ok()
                    .expect("Failed to read line");
                self.memory[self.pointer] = input.bytes().next().expect("no byte read");
            }
            '[' => {
                if self.memory[self.pointer] == 0 {
                    self.instruction = StandardBrainfuck::matching_bracket(
                        &self.instruction_stack,
                        self.instruction,
                    )
                    .expect(
                        "Matching bracket could not be found at instruction number {instruction}",
                    );
                }
            }
            ']' => {
                if self.memory[self.pointer] != 0 {
                    self.instruction = StandardBrainfuck::matching_bracket_reversed(
                        &self.instruction_stack,
                        self.instruction,
                    )
                    .expect(
                        "Matching bracket could not be found at instruction number {instruction}",
                    );
                }
            }
            _ => {}
        }
        self.instruction += 1;
    }
    fn init_interpretation(&mut self) {
        while self.instruction != self.instruction_stack.len() {
            self.next_instruction()
        }
    }
}
