use crate::util;
use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

/// A Standard brainfuck interpreter
///
/// This provides an implementation of the `StdProgram` trait for the Brainfuck
/// language.
/// Uses a Double Queue Vector to represent the memory tape and a Vector to store the
/// Brainfuck instructions.
/// Allows for unlimited memory and memory wrapping
/// Consists of X instructions:
/// + - Increment the memory cell under the pointer
/// - - Decrement the memory cell under the pointer
/// > - Move the pointer to the right
/// < - Move the pointer to the left
/// [ - Jump past the matching bracket if the cell under the pointer is 0
/// ] - Jump back to the matching bracket
/// . - Output the character signified by the cell at the pointer
/// , - Input a character and store it in the cell at the pointer
#[derive(Default)]
pub struct StandardBrainfuck {
    pub memory: VecDeque<u8>,
    pub pointer: usize,
    pub instruction: usize,
    pub instruction_stack: Vec<char>,
}

pub trait StdProgram: StdOperations {
    fn new(instruction_set: Vec<char>) -> Self;
    fn next_instruction(&mut self) -> &mut Self;
    fn run_full_stack(&mut self);
}
pub trait StdOperations {
    fn op_ptr_left(&mut self);
    fn op_ptr_right(&mut self);
    fn op_add_to_cell(&mut self);
    fn op_sub_from_cell(&mut self);
    fn op_print_cell_as_char(&self);
    fn op_input_to_cell(&mut self);
    fn op_jump_forwards(&mut self);
    fn op_jump_backwards(&mut self);
}

impl StdProgram for StandardBrainfuck {
    fn new(instruction_stack: Vec<char>) -> Self {
        return Self {
            instruction: 0,
            pointer: 0,
            memory: VecDeque::from([0x00]),
            instruction_stack: instruction_stack,
        };
    }
    fn next_instruction(&mut self) -> &mut Self {
        match self.instruction_stack[self.instruction] {
            '+' => self.op_add_to_cell(),
            '-' => self.op_sub_from_cell(),
            '>' => self.op_ptr_left(),
            '<' => self.op_ptr_right(),
            '.' => self.op_print_cell_as_char(),
            ',' => self.op_input_to_cell(),
            '[' => self.op_jump_forwards(),
            ']' => self.op_jump_backwards(),
            _ => {}
        }
        self.instruction += 1;
        return self;
    }
    fn run_full_stack(&mut self) {
        while self.instruction != self.instruction_stack.len() {
            self.next_instruction();
        }
    }
}

impl StdOperations for StandardBrainfuck {
    fn op_add_to_cell(&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
    }
    fn op_sub_from_cell(&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
    }
    fn op_ptr_left(&mut self) {
        if self.pointer + 1 >= self.memory.len() {
            self.memory.push_back(0)
        }
        self.pointer += 1;
    }
    fn op_ptr_right(&mut self) {
        if self.pointer <= 0 {
            self.memory.push_front(0);
        }
        self.pointer -= 1;
    }
    fn op_print_cell_as_char(&self) {
        print!("{}", self.memory[self.pointer] as char);
        stdout().flush().unwrap();
    }
    fn op_input_to_cell(&mut self) {
        let mut input: String = String::new();
        stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        self.memory[self.pointer] = input.bytes().next().expect("no byte read");
    }
    fn op_jump_forwards(&mut self) {
        if self.memory[self.pointer] == 0 {
            self.instruction = util::matching::find_matching_characters(
                &self.instruction_stack,
                '[',
                ']',
                self.instruction,
            )
            .expect("Matching bracket could not be found at instruction number {instruction}");
        }
    }
    fn op_jump_backwards(&mut self) {
        if self.memory[self.pointer] != 0 {
            self.instruction = util::matching::find_matching_characters_reversed(
                &self.instruction_stack,
                '[',
                ']',
                self.instruction,
            )
            .expect("Matching bracket could not be found at instruction number {instruction}");
        }
    }
}
