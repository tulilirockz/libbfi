use crate::interpreter::generic_dialect::*;
use crate::interpreter::macros::*;
use crate::token::*;

/// A Ook brainfuck derivative interpreter
///
/// Consists of 8 instructions:
///
///     + - Increment the memory cell under the pointer
///     - - Decrement the memory cell under the pointer
///     > - Move the pointer to the right
///     < - Move the pointer to the left
///     [ - Jump past the matching bracket if the cell under the pointer is 0
///     ] - Jump back to the matching bracket
///     . - Output the character signified by the cell at the pointer
///     , - Input a character and store it in the cell at the pointer
///
/// Example:
///
/// ```bf
/// Ook. Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook! Ook? Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook? Ook! Ook! Ook? Ook! Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook! Ook. Ook. Ook. Ook! Ook.
/// ```
pub struct Ook;
/// A Blub brainfuck derivative interpreter
///
/// Consists of 8 instructions:
///
///     Blub. Blub. - Increment the memory cell under the pointer
///     Blub! Blub! - Decrement the memory cell under the pointer
///     Blub? Blub. - Move the pointer to the right
///     Blub. Blub? - Move the pointer to the left
///     Blub! Blub. - Jump past the matching bracket if the cell under the pointer is 0
///     Blub. Blub! - Jump back to the matching bracket
///     Blub! Blub? - Output the character signified by the cell at the pointer
///     Blub? Blub! - Input a character and store it in the cell at the pointer
///
/// Example:
///
/// ```bf
/// Blub. Blub? Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub! Blub? Blub? Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub? Blub! Blub! Blub? Blub! Blub? Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub! Blub. Blub. Blub. Blub! Blub.
/// ```
pub struct Blub;
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

impl Memory<Ook> {
    multi_char_tokens!(2, ".!?", "..", "!!", "?.", ".?", "!.", ".!", "!?", "?!");
}

impl Memory<Blub> {
    multi_char_tokens!(2, ".!?", "..", "!!", "?.", ".?", "!.", ".!", "!?", "?!");
}

impl<Dialect> Memory<Dialect> {
    to_other_dialect!(ook, Ook);
    to_other_dialect!(blub, Blub);
    to_other_dialect!(custom, Custom);
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
        return Ok(match self {
            BFToken::CellAdd => custom.add,
            BFToken::CellSubtract => custom.subtract,
            BFToken::PtrLeft => custom.left,
            BFToken::PtrRight => custom.right,
            BFToken::Print => custom.print,
            BFToken::Input => custom.input,
            BFToken::JumpForwards => custom.jump_forwards,
            BFToken::JumpBackwards => custom.jump_backwards,
            _ => return Err(TokenParseError),
        });
    }
}

impl Custom {
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
        return Ok(final_object);
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

impl Memory<Custom> {
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
                return {
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
                };
            })
            .filter(|x| *x != BFToken::NoOP)
            .collect();
        self.instruction_stack.append(&mut thing);
        return Ok(self);
    }
}
