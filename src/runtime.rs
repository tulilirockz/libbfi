#![doc = r"Traits and definitions for building a brainfuck interpreter"]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Sharing memory should work like this:
//! let env = BrainfuckRuntime::new().add_tokens(program).run_full_stack().clean_env()
//! let runtime = BoofRuntime::new().add_tokens(program);
//! runtime.memory = env.memory;
//! runtime.run_full_stack().clean_env()

use crate::token::BFToken;
use std::{io::BufRead, io::Write};

pub trait Operator {
    fn op_ptr_left(&mut self);
    fn op_ptr_right(&mut self);
    fn op_add_to_cell(&mut self);
    fn op_sub_from_cell(&mut self);
    fn op_print_cell_as_char(&self, writer: &mut impl Write);
    fn op_input_to_cell(&mut self, reader: &mut impl BufRead);
    fn op_jump_forwards(&mut self);
    fn op_jump_backwards(&mut self);
}

pub trait Runner: Clone + Sized {
    /// Adds a sequence of tokens (instructions) to the runtime's instruction stack.
    fn add_tokens(&mut self, token_stream: Vec<BFToken>) -> &mut Self;
    /// Resets the runtime environment to its initial state: instruction and pointer are set to zero,
    /// memory is cleared (all cells set to 0), and the instruction stack is emptied.
    fn clean_env(&mut self) -> &mut Self;
    /// Executes the next instruction from the instruction stack.
    fn next_instruction(&mut self, reader: &mut impl BufRead, writer: &mut impl Write)
        -> &mut Self;
    /// Executes all instructions in the stack until the end is reached.
    fn run_full_stack(&mut self, reader: &mut impl BufRead, writer: &mut impl Write) -> &mut Self;
}
