#[cfg(test)]
use libbfi::interpreter::generic_dialect::Memory;
use libbfi::{interpreter::languages::Ook, token::BFToken};

#[test]
fn test_adding_characters() {
    let mut binding = Memory::new();
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

#[test]
fn test_to_ook() {
    let mut bind = Memory::new();
    let tokens: &mut Memory = bind.add_tokens("+a-b>c<d[e]f".chars()).expect("amogus");

    let mut ook: Memory<Ook> = tokens.to_ook();
    let ooktoken: &mut Memory<Ook> = ook
        .add_tokens::<Ook>("Ook. Ook. Ook. Ook.")
        .expect("msFAILg");

    assert_eq!(
        ooktoken.instruction_stack,
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
