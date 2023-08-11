#![doc = r"Support for trivial brainfuck languages"]
#![cfg_attr(docsrs, feature(doc_cfg))]

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
/// use libbfi::prelude::*;
/// use std::io::{stdin,stdout};
///
/// let program: String = String::from(">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.");
/// let mut std_brainfuck_app = Memory::new();
/// std_brainfuck_app.add_tokens(Brainfuck::to_tokens(program)
///      .expect("Failed parsing program"))
///      .run_full_stack(&mut stdin().lock(), &mut stdout())
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
/// use libbfi::prelude::*;
/// use std::io::{stdin,stdout};
///
/// let program: String = String::from("Ook. Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook! Ook? Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook? Ook! Ook! Ook? Ook! Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook! Ook. Ook. Ook. Ook! Ook.");
/// let mut monke = Memory::new();
///
/// monke.add_tokens(Ook::to_tokens(program).expect("Failed parsing program"))
///      .run_full_stack(&mut stdin().lock(), &mut stdout())
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
/// use libbfi::prelude::*;
/// use std::io::{stdin,stdout};
///
/// let program: String = String::from("Blub. Blub? Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub! Blub? Blub? Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub? Blub! Blub! Blub? Blub! Blub? Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub. Blub! Blub. Blub. Blub. Blub! Blub.");
/// let mut fishe = Memory::new();
///
/// fishe.add_tokens(Blub::to_tokens(program).expect("Failed parsing program"))
///      .run_full_stack(&mut stdin().lock(), &mut stdout())
///      .clean_env();
/// ```
pub struct Blub;

impl Tokenizer for Brainfuck {
    single_char_tokenizer!(char, '+', '-', '<', '>', '.', ',', '[', ']');
    token_to_string!("+", "-", "<", ">", ".", ",", "[", "]");
}

impl Tokenizer for Ook {
    multi_char_tokenizer!(str, 2, ".!?", "..", "!!", "?.", ".?", "!.", ".!", "!?", "?!");
    token_to_string!(
        "Ook. Ook.",
        "Ook! Ook!",
        "Ook? Ook.",
        "Ook. Ook?",
        "Ook! Ook.",
        "Ook. Ook!",
        "Ook! Ook?",
        "Ook? Ook!"
    );
}

impl Tokenizer for Blub {
    multi_char_tokenizer!(str, 2, ".!?", "..", "!!", "?.", ".?", "!.", ".!", "!?", "?!");
    token_to_string!(
        "Blub. Blub.",
        "Blub! Blub!",
        "Blub? Blub.",
        "Blub. Blub?",
        "Blub! Blub.",
        "Blub. Blub!",
        "Blub! Blub?",
        "Blub? Blub!"
    );
}
