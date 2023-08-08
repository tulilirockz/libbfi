#![doc = r"Contains everything necessary to write support for other brainfuck-like languages"]

use crate::interpreter::bf::BrainfuckMemory;
use crate::languages::builtin::*;
use crate::languages::custom::Custom;
use crate::macros::token_conversion::to_other_dialect_declaration;
use std::io::{BufRead, Write};

pub trait BrainfuckOperations {
    fn op_ptr_left(&mut self);
    fn op_ptr_right(&mut self);
    fn op_add_to_cell(&mut self);
    fn op_sub_from_cell(&mut self);
    fn op_print_cell_as_char(&self, writer: &mut impl Write);
    fn op_input_to_cell(&mut self, reader: &mut impl BufRead);
    fn op_jump_forwards(&mut self);
    fn op_jump_backwards(&mut self);
}

pub trait BrainfuckTranslator {
    to_other_dialect_declaration!(bf, Brainfuck);
    to_other_dialect_declaration!(ook, Ook);
    to_other_dialect_declaration!(blub, Blub);
    to_other_dialect_declaration!(custom, Custom);
}

pub trait BrainfuckParser {
    fn clean_env(&mut self) -> &mut Self;
    fn next_instruction(&mut self, reader: &mut impl BufRead, writer: &mut impl Write)
        -> &mut Self;
    fn run_full_stack(&mut self, reader: &mut impl BufRead, writer: &mut impl Write) -> &mut Self;
}
