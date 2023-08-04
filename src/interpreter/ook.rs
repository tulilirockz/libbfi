use crate::interpreter::std_bf::{StdOperators, StdParser};
use crate::util::matching::{find_matching_substring, find_matching_substring_reversed};
use std::io::{stdin, stdout, Write};

pub struct Ook {
    memory: [u8; 30000],
    pointer: usize,
    instruction: usize,
    instruction_stack: Vec<char>,
}

impl StdParser for Ook {
    fn new(instruction_stack: &impl AsRef<str>) -> Self {
        return Self {
            instruction: 0,
            pointer: 0,
            memory: [0x00; 30000],
            instruction_stack: instruction_stack.as_ref().chars().collect(),
        };
    }
    fn next_instruction_in_stack(&mut self) -> &mut Self {
        match format!(
            "{}{}",
            self.instruction_stack[self.instruction],
            self.instruction_stack[self.instruction + 1]
        )
        .as_str()
        {
            ".." => self.op_add_to_cell(),
            "!!" => self.op_sub_from_cell(),
            "?." => self.op_ptr_left(),
            ".?" => self.op_ptr_right(),
            "!." => self.op_print_cell_as_char(),
            ".!" => self.op_input_to_cell(),
            "!?" => self.op_jump_forwards(),
            "?!" => self.op_jump_backwards(),
            _ => { /*Gave a banana to the pointer!*/ }
        }
        self.instruction += 2;
        return self;
    }
    fn run_full_stack(&mut self) {
        while self.instruction != self.instruction_stack.len() {
            self.next_instruction_in_stack();
        }
    }
    fn filter_characters(&mut self) -> Result<&mut Self, String> {
        self.instruction_stack
            .retain(|x| ['.', '!', '?', '!'].contains(&x));
        if !self.instruction_stack.len() % 2 == 0 {
            return Err(String::from(
                "Failed to properly parse string! - Number of instructions is not a multiple of two",
            ));
        }
        return Ok(self);
    }
}

impl StdOperators for Ook {
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
            self.instruction =
                find_matching_substring(&self.instruction_stack, "!?", "?!", self.instruction)
                    .expect(
                        "Matching bracket could not be found at instruction number {instruction}",
                    );
        }
    }
    fn op_jump_backwards(&mut self) {
        if self.memory[self.pointer] != 0 {
            self.instruction = find_matching_substring_reversed(
                &self.instruction_stack,
                "!?",
                "?!",
                self.instruction,
            )
            .expect("Matching bracket could not be found at instruction number {instruction}");
        }
    }
}
