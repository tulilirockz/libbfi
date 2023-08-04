use crate::util;

use std::io::{stdin, stdout, Write};

/// A Standard brainfuck interpreter
///
/// This provides an implementation of the `StdProgram` trait for the Brainfuck
/// language.
/// Consists of 8 instructions:
/// + - Increment the memory cell under the pointer
/// - - Decrement the memory cell under the pointer
/// > - Move the pointer to the right
/// < - Move the pointer to the left
/// [ - Jump past the matching bracket if the cell under the pointer is 0
/// ] - Jump back to the matching bracket
/// . - Output the character signified by the cell at the pointer
/// , - Input a character and store it in the cell at the pointer
pub struct StandardBrainfuck {
    memory: [u8; 30000],
    pointer: usize,
    instruction: usize,
    instruction_stack: Vec<char>,
}

pub trait StdParser {
    fn new(instruction_stack: &impl AsRef<str>) -> Self;
    fn next_instruction_in_stack(&mut self) -> &mut Self;
    fn run_full_stack(&mut self);
    fn filter_characters(&mut self) -> Result<&mut Self, String>;
}

pub trait StdOperators {
    fn op_ptr_left(&mut self);
    fn op_ptr_right(&mut self);
    fn op_add_to_cell(&mut self);
    fn op_sub_from_cell(&mut self);
    fn op_print_cell_as_char(&self);
    fn op_input_to_cell(&mut self);
    fn op_jump_forwards(&mut self);
    fn op_jump_backwards(&mut self);
}

impl StdParser for StandardBrainfuck {
    fn new(instruction_stack: &impl AsRef<str>) -> Self {
        return Self {
            instruction: 0,
            pointer: 0,
            memory: [0x00; 30000],
            instruction_stack: instruction_stack.as_ref().chars().collect(),
        };
    }
    fn next_instruction_in_stack(&mut self) -> &mut Self {
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
            self.next_instruction_in_stack();
        }
    }
    fn filter_characters(&mut self) -> Result<&mut Self, String> {
        self.instruction_stack
            .retain(|x| ['>', '<', '[', ']', '.', ',', '+', '-'].contains(&x));
        return Ok(self);
    }
}

impl StdOperators for StandardBrainfuck {
    fn op_add_to_cell(&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
    }
    fn op_sub_from_cell(&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
    }
    fn op_ptr_left(&mut self) {
        self.pointer += 1;
    }
    fn op_ptr_right(&mut self) {
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
