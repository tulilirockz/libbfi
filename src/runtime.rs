#![doc = r"Traits and definitions for building a brainfuck interpreter"]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Sharing memory should work like this:
//! let env = BrainfuckRuntime::new().add_tokens(program).run_full_stack().clean_env()
//! let runtime = BoofRuntime::new().add_tokens(program);
//! runtime.memory = env.memory;
//! runtime.run_full_stack().clean_env()

use std::{io::BufRead, io::Write};
use crate::token::BFToken;

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
    fn add_tokens(&mut self, token_stream: Vec<BFToken>) -> &mut Self;
    fn clean_env(&mut self) -> &mut Self;
    fn next_instruction(&mut self, reader: &mut impl BufRead, writer: &mut impl Write)
        -> &mut Self;
    fn run_full_stack(&mut self, reader: &mut impl BufRead, writer: &mut impl Write) -> &mut Self;
}
