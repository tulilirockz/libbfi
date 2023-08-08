//! A library for interpreting and parsing Brainfuck code inspired by libbf.
//!
//! Includes support for regular Brainfuck and optinally any trivial-implementation of it.
//! Since any program interpreted by this library gets translated to tokens, translating one program to another shouldn't be an issue. Brainfuck -> Ook, Ook -> Blub, Blub -> Brainfuck, for example. And you can also run them interchangeably, by adding tokens in any of their states.
//!
//! ## Example Program
//!
//! ```rust
//! use libbfi::languages::builtin::*;
//! use libbfi::prelude::*;
//! use std::io::{stdin,stdout};
//!
//!     let program: &str = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.";
//!
//!     let program_ook: &str =
//!         "Ook. Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook.
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
//!      Ook! Ook. Ook. Ook? Ook. Ook? Ook. Ook. Ook! Ook.";
//!
//!     let mut std_brainfuck_app = BrainfuckMemory::new();
//!
//!     std_brainfuck_app
//!         .add_tokens(program.chars())
//!         .expect("Failed parsing program")
//!         .run_full_stack(&mut stdin().lock(), &mut stdout())
//!         .clean_env();
//!
//!     let mut ook_app = std_brainfuck_app.to_ook();
//!
//!     ook_app
//!         .add_tokens::<Ook>(program_ook)
//!         .expect("Failed parsing program")
//!         .run_full_stack(&mut stdin().lock(), &mut stdout());
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod interpreter {
    pub mod bf;
    pub mod bf_features;
}

pub mod macros {
    pub mod token_conversion;
    pub mod token_gen;
}

pub mod languages {
    pub mod builtin;
    pub mod custom;
}

pub mod matching;
pub mod token;

// Import this for necessary support to run the main Brainfuck interpreter
pub mod prelude {
    pub use crate::interpreter::bf::*;
    pub use crate::interpreter::bf_features::*;
    pub use crate::matching;
    pub use crate::token::{BFToken, TokenParseError};
}
