use crate::interpreter::languages::*;
use crate::interpreter::macros::*;
use crate::matching::*;
use crate::parser::*;
use crate::token::*;

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
pub struct Brainfuck;

pub struct Memory<Dialect = Brainfuck> {
    pub memory: [u8; 30000],
    pub pointer: usize,
    pub instruction: usize,
    pub instruction_stack: Vec<BFToken>,
    state: std::marker::PhantomData<Dialect>,
}

impl Memory {
    pub fn new() -> Self {
        return Self {
            instruction: 0,
            pointer: 0,
            memory: [0x00; 30000],
            instruction_stack: Vec::new(),
            state: Default::default(),
        };
    }
}

impl Memory<Brainfuck> {
    single_char_tokens!('+', '-', '<', '>', '.', ',', '[', ']');
}

impl<Dialect> Memory<Dialect> {
    to_other_dialect!(bf, Brainfuck);
    to_other_dialect!(ook, Ook);
    pub fn clean_env(&mut self) -> &mut Self {
        self.instruction = 0;
        self.pointer = 0;
        self.memory = [0x00; 30000];
        self.instruction_stack = Vec::new();
        return self;
    }
    pub fn next_instruction(&mut self) -> &mut Self {
        match self.instruction_stack[self.instruction] {
            BFToken::CellAdd => self.op_add_to_cell(),
            BFToken::CellSubtract => self.op_sub_from_cell(),
            BFToken::PtrLeft => self.op_ptr_left(),
            BFToken::PtrRight => self.op_ptr_right(),
            BFToken::Print => self.op_print_cell_as_char(),
            BFToken::Input => self.op_input_to_cell(),
            BFToken::JumpForwards => self.op_jump_forwards(),
            BFToken::JumpBackwards => self.op_jump_backwards(),
            _ => {}
        }
        self.instruction += 1;
        return self;
    }
    pub fn run_full_stack(&mut self) -> &mut Self {
        while self.instruction != self.instruction_stack.len() {
            self.next_instruction();
        }
        return self;
    }
}

impl<T> StdOperators for Memory<T> {
    fn op_add_to_cell(&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
    }
    fn op_sub_from_cell(&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
    }
    fn op_ptr_left(&mut self) {
        self.pointer -= 1;
    }
    fn op_ptr_right(&mut self) {
        self.pointer += 1;
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
            self.instruction = find_matching(
                &self.instruction_stack,
                BFToken::JumpForwards,
                BFToken::JumpBackwards,
                self.instruction,
                IteratorOrder::FrontToBack,
            )
            .expect("Matching bracket could not be found at instruction number {instruction}");
        }
    }
    fn op_jump_backwards(&mut self) {
        if self.memory[self.pointer] != 0 {
            self.instruction = find_matching(
                &self.instruction_stack,
                BFToken::JumpForwards,
                BFToken::JumpBackwards,
                self.instruction,
                IteratorOrder::BackToFront,
            )
            .expect("Matching bracket could not be found at instruction number {instruction}");
        }
    }
}
