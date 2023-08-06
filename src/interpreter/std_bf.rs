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
pub struct StdBrainfuck {
    pub memory: [u8; 30000],
    pub pointer: usize,
    pub instruction: usize,
    pub instruction_stack: Vec<BFToken>,
}
impl_generic_bf_op!(StdBrainfuck);

impl Parser for StdBrainfuck {
    fn new() -> Self {
        return Self {
            instruction: 0,
            pointer: 0,
            memory: [0x00; 30000],
            instruction_stack: Vec::new(),
        };
    }
    fn add_tokens<T>(&mut self, iterator: T) -> Result<&mut Self, TokenParseError>
    where
        T: IntoIterator<Item = char>,
    {
        let mut thing: Vec<BFToken> = iterator
            .into_iter()
            .map(|token| {
                return match token {
                    '+' => BFToken::CellAdd,
                    '-' => BFToken::CellSubtract,
                    '>' => BFToken::PtrRight,
                    '<' => BFToken::PtrLeft,
                    '.' => BFToken::Print,
                    ',' => BFToken::Input,
                    '[' => BFToken::JumpForwards,
                    ']' => BFToken::JumpBackwards,
                    _ => BFToken::NoOP,
                };
            })
            .filter(|x| *x != BFToken::NoOP)
            .collect();
        self.instruction_stack.append(&mut thing);
        return Ok(self);
    }
    fn clean_env(&mut self) -> &mut Self {
        self.instruction = 0;
        self.pointer = 0;
        self.memory = [0x00; 30000];
        self.instruction_stack = Vec::new();
        return self;
    }
    fn next_instruction(&mut self) -> &mut Self {
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
    fn run_full_stack(&mut self) {
        while self.instruction != self.instruction_stack.len() {
            self.next_instruction();
        }
    }
}

impl BFToken {
    #[allow(dead_code)]
    fn to_bf_char(&self) -> Result<char, TokenParseError> {
        return Ok(match self {
            BFToken::CellAdd => '+',
            BFToken::CellSubtract => '-',
            BFToken::PtrLeft => '<',
            BFToken::PtrRight => '>',
            BFToken::Print => '.',
            BFToken::Input => ',',
            BFToken::JumpForwards => '[',
            BFToken::JumpBackwards => ']',
            _ => return Err(TokenParseError),
        });
    }
}
