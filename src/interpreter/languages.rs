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

impl Memory<Ook> {
    multi_char_tokens!(2, ".!?", "..", "!!", "?.", ".?", "!.", ".!", "!?", "?!");
}

impl Memory<Blub> {
    multi_char_tokens!(2, ".!?", "..", "!!", "?.", ".?", "!.", ".!", "!?", "?!");
}

impl<Dialect> Memory<Dialect> {
    to_other_dialect!(ook, Ook);
    to_other_dialect!(blub, Blub);
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
}
