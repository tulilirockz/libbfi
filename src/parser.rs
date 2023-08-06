pub trait Parser {
    fn new() -> Self;
    fn add_tokens<T>(&mut self, iterator: T) -> Result<&mut Self, TokenParseError>
    where
        T: IntoIterator<Item = char>;
    fn next_instruction(&mut self) -> &mut Self;
    fn run_full_stack(&mut self);
    fn clean_env(&mut self) -> &mut Self;
}

pub trait StdOperators {
    fn op_ptr_left(&mut self);
    fn op_ptr_right(&mut self);
    fn op_add_to_cell(&mut self);
    fn op_sub_from_cell(&mut self);
    fn op_print_cell_as_char(&self);
    fn op_input_to_cell(&mut self);
    fn op_jump_forwards(&mut self);
    fn op_jump_backwards(&mut self);
}

macro_rules! impl_generic_bf_op {
    ($type: ident) => {
        impl StdOperators for $type {
            fn op_add_to_cell(&mut self) {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
            }
            fn op_sub_from_cell(&mut self) {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
            }
            fn op_ptr_left(&mut self) {
                self.pointer -= 1;
            }
            fn op_ptr_right(&mut self) {
                self.pointer += 1;
            }
            fn op_print_cell_as_char(&self) {
                print!("{}", self.memory[self.pointer] as char);
                stdout().flush().unwrap();
            }
            fn op_input_to_cell(&mut self) {
                let mut input: String = String::new();
                stdin()
                    .read_line(&mut input)
                    .ok()
                    .expect("Failed to read line");
                self.memory[self.pointer] = input.bytes().next().expect("no byte read");
            }
            fn op_jump_forwards(&mut self) {
                if self.memory[self.pointer] == 0 {
                    self.instruction = find_matching(
                        &self.instruction_stack,
                        BFToken::JumpForwards,
                        BFToken::JumpBackwards,
                        self.instruction,
                        IteratorOrder::FrontToBack,
                    )
                    .expect(
                        "Matching bracket could not be found at instruction number {instruction}",
                    );
                }
            }
            fn op_jump_backwards(&mut self) {
                if self.memory[self.pointer] != 0 {
                    self.instruction = find_matching(
                        &self.instruction_stack,
                        BFToken::JumpForwards,
                        BFToken::JumpBackwards,
                        self.instruction,
                        IteratorOrder::BackToFront,
                    )
                    .expect(
                        "Matching bracket could not be found at instruction number {instruction}",
                    );
                }
            }
        }
    };
}

pub(crate) use bf_session_type;
pub(crate) use impl_generic_bf_op;

use crate::token::TokenParseError;
