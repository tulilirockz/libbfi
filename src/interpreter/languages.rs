use crate::interpreter::generic_dialect::*;
use crate::interpreter::macros::*;
use crate::token::*;

pub struct Ook;
pub struct Blub;

impl Memory<Ook> {
    multi_char_tokens!(2, ".!?", "..", "!!", "?.", ".?", "!.", ".!", "!?", "?!");
}

impl Memory<Blub> {
    multi_char_tokens!(2, ".!?", "..", "!!", "?.", ".?", "!.", ".!", "!?", "?!");
}

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
}
