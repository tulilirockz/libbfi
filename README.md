# LibBFI

A library for interpreting and parsing Brainfuck code. Includes support for the Ook! dialect and possibly more TODO!

```rust
use libbfi::interpreter::std_bf::*;

fn main() {
    let program: &str = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.";

    StandardBrainfuck::new(&program)
        .filter_characters()
        .expect("Failed parsing characters")
        .run_full_stack();
}
```