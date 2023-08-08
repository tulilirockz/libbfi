#![doc = r"Includes definitions for running trivial brainfuck programs (+ Brainfuck definition)"]

use crate::{
    languages::{builtin::*, custom::Custom},
    macros::token_conversion::to_other_dialect,
    matching::*,
    prelude::*,
};

use std::io::{BufRead, Write};

#[derive(Clone, Debug)]
pub struct BrainfuckMemory<Dialect = Brainfuck> {
    pub memory: [u8; 30000],
    pub pointer: usize,
    pub instruction: usize,
    pub instruction_stack: Vec<BFToken>,
    pub state: std::marker::PhantomData<Dialect>,
}

impl BrainfuckMemory {
    pub fn new() -> Self {
        Self {
            instruction: 0,
            pointer: 0,
            memory: [0x00; 30000],
            instruction_stack: Vec::new(),
            state: Default::default(),
        }
    }
}

impl Default for BrainfuckMemory {
    fn default() -> Self {
        Self {
            instruction: 0,
            pointer: 0,
            memory: [0x00; 30000],
            instruction_stack: Vec::new(),
            state: Default::default(),
        }
    }
}

impl<Dialect> BrainfuckTranslator for BrainfuckMemory<Dialect> {
    to_other_dialect!(bf, Brainfuck);
    to_other_dialect!(ook, Ook);
    to_other_dialect!(blub, Blub);
    to_other_dialect!(custom, Custom);
}

impl<Dialect> BrainfuckParser for BrainfuckMemory<Dialect> {
    fn clean_env(&mut self) -> &mut Self {
        self.instruction = 0;
        self.pointer = 0;
        self.memory = [0x00; 30000];
        self.instruction_stack = Vec::new();
        self
    }
    fn next_instruction(
        &mut self,
        reader: &mut impl BufRead,
        writer: &mut impl Write,
    ) -> &mut Self {
        match self.instruction_stack[self.instruction] {
            BFToken::CellAdd => self.op_add_to_cell(),
            BFToken::CellSubtract => self.op_sub_from_cell(),
            BFToken::PtrLeft => self.op_ptr_left(),
            BFToken::PtrRight => self.op_ptr_right(),
            BFToken::Print => self.op_print_cell_as_char(writer),
            BFToken::Input => self.op_input_to_cell(reader),
            BFToken::JumpForwards => self.op_jump_forwards(),
            BFToken::JumpBackwards => self.op_jump_backwards(),
            _ => {}
        }
        self.instruction += 1;
        self
    }
    fn run_full_stack(&mut self, reader: &mut impl BufRead, writer: &mut impl Write) -> &mut Self {
        while self.instruction != self.instruction_stack.len() {
            self.next_instruction(reader, writer);
        }
        self
    }
}

impl<T> BrainfuckOperations for BrainfuckMemory<T> {
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
    fn op_print_cell_as_char(&self, writer: &mut impl Write) {
        write!(writer, "{}", self.memory[self.pointer] as char)
            .expect("Error when writing data to writer");
        writer.flush().unwrap();
    }
    fn op_input_to_cell(&mut self, reader: &mut impl BufRead) {
        let mut input: String = String::new();
        reader.read_line(&mut input).expect("Failed to read line");
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
