#[cfg(test)]
use libbfi::prelude::*;

const DEFAULT_TEST_ERROR: &str = "Failed to read tokens";

#[test]
fn test_adding_characters() {
    let mut runtime = BrainfuckRuntime::new();
    runtime
        .add_tokens(Brainfuck::to_tokens(String::from("+a-b>c<d[e]f")).expect(DEFAULT_TEST_ERROR));

    assert_eq!(
        runtime.instruction_stack,
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

#[test]
fn test_to_ook() {
    let mut memory = BrainfuckRuntime::new();
    memory
        .add_tokens(Brainfuck::to_tokens(String::from("+a-b>c<d[e]f")).expect(DEFAULT_TEST_ERROR))
        .add_tokens(Ook::to_tokens(String::from("Ook. Ook. Ook. Ook.")).expect(DEFAULT_TEST_ERROR));

    assert_eq!(
        memory.instruction_stack,
        vec![
            BFToken::CellAdd,
            BFToken::CellSubtract,
            BFToken::PtrRight,
            BFToken::PtrLeft,
            BFToken::JumpForwards,
            BFToken::JumpBackwards,
            BFToken::CellAdd,
            BFToken::CellAdd
        ]
    )
}
