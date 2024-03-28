use crate::{
    matching::{find_matching, IteratorOrder},
    runtime::{Operator, Runner},
    token::BFToken,
};

use std::io::{BufRead, Write};

#[derive(Debug, Clone)]
pub struct BrainfuckRuntime {
    pub pointer: usize,
    pub instruction: usize,
    pub instruction_stack: Vec<BFToken>,
    pub memory: Vec<u8>,
}

impl BrainfuckRuntime {
    /// Constructs a new `BrainfuckRuntime` instance with a default tape length of 30,000 cells.
    /// Each cell is initialized to 0. This default size is typically sufficient for many Brainfuck programs.
    pub fn new() -> Self {
        Self::with_memory_size(30_000)
    }

    /// Constructs a `BrainfuckRuntime` with a specified memory size (tape length).
    /// Initializes the tape with `size` cells, each set to 0. Allows for the creation of a runtime
    /// with custom memory size, tailored to the needs of specific Brainfuck programs.
    pub fn with_memory_size(size: usize) -> Self {
        Self {
            instruction: 0,
            pointer: 0,
            memory: vec![0x00; size],
            instruction_stack: Vec::new(),
        }
    }
}

impl Default for BrainfuckRuntime {
    fn default() -> Self {
        BrainfuckRuntime::new()
    }
}

impl Runner for BrainfuckRuntime {
    fn clean_env(&mut self) -> &mut Self {
        self.instruction = 0;
        self.pointer = 0;
        self.memory = vec![0x00; self.memory.len()];
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
    fn add_tokens(&mut self, tokens: Vec<BFToken>) -> &mut Self {
        self.instruction_stack.extend(tokens);
        self
    }
}

impl Operator for BrainfuckRuntime {
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

        reader
            .read_to_string(&mut input)
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
