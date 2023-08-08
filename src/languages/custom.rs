#![doc = r"Support for custom languages defined by the user"]

use crate::{prelude::BrainfuckMemory, token::*};

pub struct Custom {
    pub add: char,
    pub subtract: char,
    pub left: char,
    pub right: char,
    pub print: char,
    pub input: char,
    pub jump_forwards: char,
    pub jump_backwards: char,
}

impl Custom {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        add: char,
        subtract: char,
        left: char,
        right: char,
        print: char,
        input: char,
        jump_forwards: char,
        jump_backwards: char,
    ) -> Result<Self, String> {
        let final_object = Self {
            add,
            subtract,
            left,
            right,
            print,
            input,
            jump_forwards,
            jump_backwards,
        };
        if final_object.has_duplicate_fields() {
            return Err(String::from("Fields have duplicates"));
        }
        Ok(final_object)
    }

    fn has_duplicate_fields(&self) -> bool {
        let fields = [
            &self.add,
            &self.subtract,
            &self.left,
            &self.right,
            &self.print,
            &self.input,
            &self.jump_forwards,
            &self.jump_backwards,
        ];

        for (i, &outer_field) in fields.iter().enumerate() {
            for &inner_field in fields.iter().skip(i + 1) {
                if outer_field == inner_field {
                    return true;
                }
            }
        }

        false
    }
}

impl BrainfuckMemory<Custom> {
    pub fn add_tokens<T>(
        &mut self,
        custom: &Custom,
        iterator: T,
    ) -> Result<&mut Self, TokenParseError>
    where
        T: IntoIterator<Item = char>,
    {
        let mut thing: Vec<BFToken> = iterator
            .into_iter()
            .map(|token| {
                if token == custom.add {
                    BFToken::CellAdd
                } else if token == custom.subtract {
                    BFToken::CellSubtract
                } else if token == custom.left {
                    BFToken::PtrLeft
                } else if token == custom.right {
                    BFToken::PtrRight
                } else if token == custom.print {
                    BFToken::Print
                } else if token == custom.input {
                    BFToken::Input
                } else if token == custom.jump_forwards {
                    BFToken::JumpForwards
                } else if token == custom.jump_backwards {
                    BFToken::JumpBackwards
                } else {
                    BFToken::NoOP
                }
            })
            .filter(|x| *x != BFToken::NoOP)
            .collect();
        self.instruction_stack.append(&mut thing);
        Ok(self)
    }
}
