#![doc = "Definition for generic brainfuck token"]
use crate::{languages::custom::Custom, macros::token_gen::*};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BFToken {
    CellAdd,
    CellSubtract,
    PtrLeft,
    PtrRight,
    Print,
    Input,
    JumpForwards,
    JumpBackwards,
    NoOP, // Should not be used! Only for internal use :)
}

#[derive(Debug)]
pub struct TokenParseError;

impl BFToken {
    token_to_type_dialect!(bf, char, '+', '-', '<', '>', '.', ',', '[', ']');
    token_to_type_dialect!(
        ook,
        String,
        String::from("Ook. Ook."),
        String::from("Ook! Ook!"),
        String::from("Ook? Ook."),
        String::from("Ook. Ook?"),
        String::from("Ook! Ook."),
        String::from("Ook. Ook!"),
        String::from("Ook! Ook?"),
        String::from("Ook? Ook!")
    );
    token_to_type_dialect!(
        blub,
        String,
        String::from("Blub. Blub."),
        String::from("Blub! Blub!"),
        String::from("Blub? Blub."),
        String::from("Blub. Blub?"),
        String::from("Blub! Blub."),
        String::from("Blub. Blub!"),
        String::from("Blub! Blub?"),
        String::from("Blub? Blub!")
    );
    pub fn to_custom(&self, custom: &Custom) -> Result<char, TokenParseError> {
        Ok(match self {
            BFToken::CellAdd => custom.add,
            BFToken::CellSubtract => custom.subtract,
            BFToken::PtrLeft => custom.left,
            BFToken::PtrRight => custom.right,
            BFToken::Print => custom.print,
            BFToken::Input => custom.input,
            BFToken::JumpForwards => custom.jump_forwards,
            BFToken::JumpBackwards => custom.jump_backwards,
            _ => return Err(TokenParseError),
        })
    }
}
