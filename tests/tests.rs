#[cfg(test)]
use libbfi::interpreter::generic_dialect::Memory;
use libbfi::{
    interpreter::languages::{Custom, Ook},
    token::BFToken,
};

#[test]
fn test_adding_characters() {
    let mut binding = Memory::new();
    let tokens = binding.add_tokens("+a-b>c<d[e]f".chars()).unwrap();

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
    let ooktoken: &mut Memory<Ook> = ook.add_tokens::<Ook>("Ook. Ook. Ook. Ook.").unwrap();

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

#[test]
fn test_custom_type() {
    let mytype = Custom {
        add: 'p',
        subtract: 'm',
        left: 'l',
        right: 'r',
        print: 'x',
        input: 'i',
        jump_backwards: 'b',
        jump_forwards: 'f',
    };

    let custom_program = "pmlrxifb";

    let hi = Memory::new();
    let mut program_runner = hi.to_custom();
    program_runner
        .add_tokens(&mytype, custom_program.chars())
        .unwrap();

    assert_eq!(
        program_runner.instruction_stack,
        vec![
            BFToken::CellAdd,
            BFToken::CellSubtract,
            BFToken::PtrLeft,
            BFToken::PtrRight,
            BFToken::Print,
            BFToken::Input,
            BFToken::JumpForwards,
            BFToken::JumpBackwards
        ]
    );
}
