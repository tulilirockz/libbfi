#![doc = "Generic macros for making support for other languages"]

macro_rules! to_other_dialect {
    ($method_name: ident, $type: ident) => {
        paste::item! {
            pub fn [< to_ $method_name >](&self) -> Memory<$type> {
                return Memory::<$type> {
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

macro_rules! single_char_tokens {
    ($add: expr, $sub: expr, $left: expr, $right: expr, $print: expr, $input: expr, $forward: expr, $backward: expr) => {
        pub fn add_tokens<T>(&mut self, iterator: T) -> Result<&mut Self, TokenParseError>
        where
            T: IntoIterator<Item = char>,
        {
            let mut thing: Vec<BFToken> = iterator
                .into_iter()
                .map(|token| {
                    return match token {
                        $add => BFToken::CellAdd,
                        $sub => BFToken::CellSubtract,
                        $right => BFToken::PtrRight,
                        $left => BFToken::PtrLeft,
                        $print => BFToken::Print,
                        $input => BFToken::Input,
                        $forward => BFToken::JumpForwards,
                        $backward => BFToken::JumpBackwards,
                        _ => BFToken::NoOP,
                    };
                })
                .filter(|x| *x != BFToken::NoOP)
                .collect();
            self.instruction_stack.append(&mut thing);
            return Ok(self);
        }
    };
}

/// This is meant to be used with brainfuck derivatives that just need single characters to work, like Blub or Ook
/// $cutoff is necessary to interpret how many characters need to be interpreted at once (e.g.: Ook. Ook. (2 '.' characters))
macro_rules! multi_char_tokens {
    ($cutoff: expr, $valid_characters: expr, $add: expr, $sub: expr, $left: expr, $right: expr, $print: expr, $input: expr, $forward: expr, $backward: expr) => {
        pub fn add_tokens<T>(&mut self, iterator: &str) -> Result<&mut Self, TokenParseError> {
            let valid_chars: Vec<char> = iterator
                .chars()
                .filter(|c| $valid_characters.contains(*c))
                .collect();

            if valid_chars.len() % $cutoff != 0 {
                return Err(TokenParseError);
            }

            let mut tokens: Vec<BFToken> = Vec::new();

            for i in (0..valid_chars.len()).step_by($cutoff) {
                tokens.push(
                    match valid_chars[i..i + $cutoff]
                        .iter()
                        .collect::<String>()
                        .as_str()
                    {
                        $add => BFToken::CellAdd,
                        $sub => BFToken::CellSubtract,
                        $left => BFToken::PtrLeft,
                        $right => BFToken::PtrRight,
                        $print => BFToken::Print,
                        $input => BFToken::Input,
                        $forward => BFToken::JumpForwards,
                        $backward => BFToken::JumpBackwards,
                        _ => BFToken::NoOP,
                    },
                );
            }

            tokens.retain(|x| *x != BFToken::NoOP);

            self.instruction_stack.append(&mut tokens);
            Ok(self)
        }
    };
}

macro_rules! token_to_type_dialect {
    ($method_name: ident, $returning_type: ident, $add: expr, $sub: expr, $left: expr, $right: expr, $print: expr, $input: expr, $forwards: expr, $backwards: expr) => {
        paste::item! {
            pub fn [< to_ $method_name>](&self) -> Result<$returning_type, TokenParseError> {
                return Ok(match self {
                    BFToken::CellAdd => $add,
                    BFToken::CellSubtract => $sub,
                    BFToken::PtrLeft => $left,
                    BFToken::PtrRight => $right,
                    BFToken::Print => $print,
                    BFToken::Input => $input,
                    BFToken::JumpForwards => $forwards,
                    BFToken::JumpBackwards => $backwards,
                    _ => return Err(TokenParseError),
                });
            }
        }
    };
}
pub(crate) use multi_char_tokens;
pub(crate) use single_char_tokens;
pub(crate) use to_other_dialect;
pub(crate) use token_to_type_dialect;
