#![doc = "Definition for generic brainfuck token"]

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BFToken {
    CellAdd,
    CellSubtract,
    PtrLeft,
    PtrRight,
    Print,
    Input,
    JumpForwards,
    JumpBackwards,
    NoOP, // Should not be used! Only for internal use :)
}

#[derive(Debug)]
pub struct TokenParseError;
