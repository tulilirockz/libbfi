//! A library for interpreting and parsing Brainfuck code inspired by libbf.
//!
//! Includes support for regular Brainfuck and optinally any trivial-implementation of it.
//! Since any program interpreted by this library gets translated to tokens, translating one program to another shouldn't be an issue. Brainfuck -> Ook, Ook -> Blub, Blub -> Brainfuck, for example. And you can also run them interchangeably, by adding tokens in any of their states.
//!
//! ## Example Program
//!
//! ```rust
//! use libbfi::interpreter::languages::*;
//! use libbfi::prelude::*;
//!
//! fn main() {
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
//!     let mut std_brainfuck_app = Memory::new();
//!
//!     std_brainfuck_app
//!         .add_tokens(program.chars())
//!         .expect("Failed parsing program")
//!         .run_full_stack()
//!         .clean_env();
//!
//!     let mut ook_app = std_brainfuck_app.to_ook();
//!
//!     ook_app
//!         .add_tokens::<Ook>(program_ook)
//!         .expect("Failed parsing program")
//!         .run_full_stack();
//! }
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod interpreter {
    pub mod generic_dialect;
    pub mod languages;
    pub mod macros;
}

pub mod matching;
pub mod token;

// Import this for necessary support to run the main Brainfuck interpreter
pub mod prelude {
    pub use crate::interpreter::generic_dialect::*;
    pub use crate::matching;
    pub use crate::token;
}
