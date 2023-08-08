#![doc = r"Support for trivial brainfuck languages"]

use crate::interpreter::bf::*;
use crate::macros::token_gen::*;
use crate::token::*;

/// A Standard brainfuck interpreter
///
/// Consists of 8 instructions:
///
/// > + - Increment the memory cell under the pointer
/// > - - Decrement the memory cell under the pointer
/// > > - Move the pointer to the right
/// > < - Move the pointer to the left
/// > [ - Jump past the matching bracket if the cell under the pointer is 0
/// > ] - Jump back to the matching bracket
/// > . - Output the character signified by the cell at the pointer
/// > , - Input a character and store it in the cell at the pointer
///
/// Example:
///
/// ```bf
///
/// ```
/// # Example:
///
/// ```rust
/// use libbfi::languages::builtin::*;
/// use libbfi::prelude::*;
///
/// let program: &str = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.";
/// let mut std_brainfuck_app = BrainfuckMemory::new();
/// std_brainfuck_app.add_tokens(program.chars())
///      .expect("Failed parsing program")
///      .run_full_stack()
///      .clean_env();
/// ```
pub struct Brainfuck;
/// A Ook brainfuck derivative interpreter
///
/// Consists of 8 instructions:
///
/// > Ook. Ook. - Increment the memory cell under the pointer
/// > Ook! Ook! - Decrement the memory cell under the pointer
/// > Ook? Ook. - Move the pointer to the right
/// > Ook. Ook? - Move the pointer to the left
/// > Ook! Ook. - Jump past the matching bracket if the cell under the pointer is 0
/// > Ook. Ook! - Jump back to the matching bracket
/// > Ook! Ook? - Output the character signified by the cell at the pointer
/// > Ook? Ook! - Input a character and store it in the cell at the pointer
///
/// # Example:
///
/// ```rust
/// use libbfi::languages::builtin::*;
/// use libbfi::prelude::*;
///
/// let program: &str = "Ook. Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook! Ook? Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook? Ook! Ook! Ook? Ook! Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook! Ook. Ook. Ook. Ook! Ook.";
/// let std_brainfuck_app = BrainfuckMemory::new();
/// let mut monke: BrainfuckMemory<Ook> = std_brainfuck_app.to_ook();
///
/// monke.add_tokens::<Ook>(program)
///      .expect("Failed parsing program")
///      .run_full_stack()
///      .clean_env();
/// ```
pub struct Ook;
/// A Blub brainfuck derivative interpreter
///
/// Consists of 8 instructions:
///
/// > Blub. Blub. - Increment the memory cell under the pointer
/// > Blub! Blub! - Decrement the memory cell under the pointer
/// > Blub? Blub. - Move the pointer to the right
/// > Blub. Blub? - Move the pointer to the left
/// > Blub! Blub. - Jump past the matching bracket if the cell under the pointer is 0
/// > Blub. Blub! - Jump back to the matching bracket
/// > Blub! Blub? - Output the character signified by the cell at the pointer
/// > Blub? Blub! - Input a character and store it in the cell at the pointer
///
///
/// # Example:
///
/// ```rust
/// use libbfi::languages::builtin::*;
/// use libbfi::prelude::*;
///
/// let program: &str = "Blub. Blub? Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub! Blub? Blub? Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub? Blub! Blub! Blub? Blub! Blub? Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub! Blub. Blub. Blub. Blub! Blub.";
/// let std_brainfuck_app = BrainfuckMemory::new();
/// let mut fishe: BrainfuckMemory<Blub> = std_brainfuck_app.to_blub();
///
/// fishe.add_tokens::<Blub>(program)
///      .expect("Failed parsing program")
///      .run_full_stack()
///      .clean_env();
/// ```
pub struct Blub;

impl BrainfuckMemory<Brainfuck> {
    single_char_tokens!('+', '-', '<', '>', '.', ',', '[', ']');
}

impl BrainfuckMemory<Ook> {
    multi_char_tokens!(2, ".!?", "..", "!!", "?.", ".?", "!.", ".!", "!?", "?!");
}

impl BrainfuckMemory<Blub> {
    multi_char_tokens!(2, ".!?", "..", "!!", "?.", ".?", "!.", ".!", "!?", "?!");
}
