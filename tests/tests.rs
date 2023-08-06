#[cfg(test)]
use libbfi::interpreter::std_bf::StdBrainfuck;
use libbfi::{parser::Parser, token::BFToken};

#[test]
fn test_adding_characters() {
    let mut binding = StdBrainfuck::new();
    let tokens = binding.add_tokens("+a-b>c<d[e]f".chars()).expect("amogus");

    assert_eq!(
        tokens.instruction_stack,
        vec![
            BFToken::CellAdd,
            BFToken::CellSubtract,
            BFToken::PtrRight,
            BFToken::PtrLeft,
            BFToken::JumpForwards,
            BFToken::JumpBackwards
        ]
    )
}
