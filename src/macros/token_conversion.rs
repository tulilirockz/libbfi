macro_rules! to_other_dialect {
    ($method_name: ident, $type: ident) => {
        paste::item! {
            fn [< to_ $method_name >](&self) -> BrainfuckMemory<$type> {
                return BrainfuckMemory::<$type> {
                    instruction: self.instruction,
                    instruction_stack: self.instruction_stack.clone(),
                    memory: self.memory,
                    pointer: self.pointer,
                    state: std::marker::PhantomData::<$type>,
                };
            }
        }
    };
}

macro_rules! to_other_dialect_declaration {
    ($method_name: ident, $type: ident) => {
        paste::item! {
            fn [< to_ $method_name >](&self) -> BrainfuckMemory<$type>;
        }
    };
}

pub(crate) use to_other_dialect;
pub(crate) use to_other_dialect_declaration;
