//! A library for interpreting and parsing Brainfuck code inspired by libbf.
//!
//! Includes support for regular Brainfuck and optionally any trivial-implementation of it.
//! Since any program interpreted by this library gets translated to tokens, translating one program to another shouldn't be an issue. Brainfuck -> Ook, Ook -> Blub, Blub -> Brainfuck, for example. And you can also run them interchangeably, by adding tokens in any of their states.
//!
//! ## Example Program
//!
//! ```rust
//! use libbfi::prelude::*;
//! use std::io::{stdin,stdout};
//!
//!     let program: String = String::from(">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.");
//!
//!     let program_ook: String =
//!         String::from("Ook. Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook.
//!      Ook. Ook. Ook. Ook. Ook! Ook? Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook.
//!      Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook? Ook! Ook! Ook? Ook! Ook? Ook.
//!      Ook! Ook. Ook. Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook.
//!      Ook. Ook. Ook! Ook? Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook?
//!      Ook! Ook! Ook? Ook! Ook? Ook. Ook. Ook. Ook! Ook. Ook. Ook. Ook. Ook. Ook. Ook.
//!      Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook! Ook. Ook! Ook. Ook. Ook. Ook. Ook.
//!      Ook. Ook. Ook! Ook. Ook. Ook? Ook. Ook? Ook. Ook? Ook. Ook. Ook. Ook. Ook. Ook.
//!      Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook! Ook? Ook? Ook. Ook. Ook.
//!      Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook? Ook! Ook! Ook? Ook! Ook? Ook. Ook! Ook.
//!      Ook. Ook? Ook. Ook? Ook. Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook.
//!      Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook! Ook? Ook? Ook. Ook. Ook.
//!      Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook.
//!      Ook. Ook? Ook! Ook! Ook? Ook! Ook? Ook. Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook.
//!      Ook? Ook. Ook? Ook. Ook? Ook. Ook? Ook. Ook! Ook. Ook. Ook. Ook. Ook. Ook. Ook.
//!      Ook! Ook. Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook.
//!      Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook!
//!      Ook! Ook. Ook. Ook? Ook. Ook? Ook. Ook. Ook! Ook.");
//!
//!     let mut std_brainfuck_app = Memory::new();
//!
//!     std_brainfuck_app
//!         .add_tokens(Brainfuck::to_tokens(program).expect("Failed parsing program"))
//!         .run_full_stack(&mut stdin().lock(), &mut stdout())
//!         .clean_env();
//!     
//!     std_brainfuck_app
//!         .add_tokens(Ook::to_tokens(program_ook).expect("Failed parsing program"))
//!         .run_full_stack(&mut stdin().lock(), &mut stdout());
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod builtin_tokenizers;
pub mod interpreter;
pub mod matching;
pub mod runtime;
pub mod token;

// Import this for necessary support to run the main Brainfuck interpreter
pub mod prelude {
    pub use crate::builtin_tokenizers::*;
    pub use crate::interpreter::*;
    pub use crate::matching;
    pub use crate::runtime::*;
    pub use crate::token::*;
}
