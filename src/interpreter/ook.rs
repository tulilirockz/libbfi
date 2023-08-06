use crate::matching::*;
use crate::parser::*;
use crate::token::*;
use std::io::{stdin, stdout, Write};

pub struct Ook {
    pub memory: [u8; 30000],
    pub pointer: usize,
    pub instruction: usize,
    pub instruction_stack: Vec<BFToken>,
}
impl_generic_bf_op!(Ook);

impl Parser for Ook {
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
        let iter_to_str: Vec<char> = iterator.into_iter().collect();
        let count = iter_to_str.len();
        if count % 2 != 0 {
            return Err(TokenParseError);
        }

        let mut tokens: Vec<BFToken> = Vec::new();

        for i in (0..count).step_by(2) {
            let token_str = format!("{}{}", iter_to_str[i], iter_to_str[i + 1]);
            tokens.push(match token_str.as_str() {
                ".." => BFToken::CellAdd,
                "!!" => BFToken::CellSubtract,
                "?." => BFToken::PtrLeft,
                ".?" => BFToken::PtrRight,
                "!." => BFToken::Print,
                ".!" => BFToken::Input,
                "!?" => BFToken::JumpForwards,
                "?!" => BFToken::JumpBackwards,
                _ => return Err(TokenParseError),
            });
        }

        self.instruction_stack.append(&mut tokens);
        Ok(self)
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
            _ => { /*Gave a banana to the pointer!*/ }
        }
        self.instruction += 2;
        return self;
    }
    fn run_full_stack(&mut self) {
        while self.instruction != self.instruction_stack.len() {
            self.next_instruction();
        }
    }
}
