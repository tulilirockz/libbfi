#![doc = "Definition for generic brainfuck token + everything necessary to write tokenizers"]

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BFToken {
    CellAdd,
    CellSubtract,
    PtrLeft,
    PtrRight,
    Print,
    Input,
    JumpForwards,
    JumpBackwards,
    NoOP, // Only for internal use
}

#[derive(Debug)]
pub struct TokenParseError;
pub trait Tokenizer {
    fn to_tokens(element: String) -> Result<Vec<BFToken>, TokenParseError>;
    fn token_to_string(element: BFToken) -> Result<String, TokenParseError>;
}

macro_rules! single_char_tokenizer {
    ($type: ty, $add: expr, $sub: expr, $left: expr, $right: expr, $print: expr, $input: expr, $forward: expr, $backward: expr) => {
        fn to_tokens(iterator: String) -> Result<Vec<BFToken>, TokenParseError> {
            Ok(iterator
                .chars()
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
                .collect())
        }
    };
}

/// This is meant to be used with brainfuck derivatives that just need single characters to work, like Blub or Ook
/// $cutoff is necessary to interpret how many characters need to be interpreted at once (e.g.: Ook. Ook. (2 '.' characters))
macro_rules! multi_char_tokenizer {
    ($type: ty, $cutoff: expr, $valid_characters: expr, $add: expr, $sub: expr, $left: expr, $right: expr, $print: expr, $input: expr, $forward: expr, $backward: expr) => {
        fn to_tokens(iterator: String) -> Result<Vec<BFToken>, TokenParseError> {
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

            Ok(tokens)
        }
    };
}

macro_rules! token_to_string {
    ($add: expr, $sub: expr, $left: expr, $right: expr, $print: expr, $input: expr, $forwards: expr, $backwards: expr) => {
        paste::item! {
            fn token_to_string(element: BFToken) -> Result<String, TokenParseError> {
                return Ok(String::from(match element {
                    BFToken::CellAdd => $add,
                    BFToken::CellSubtract => $sub,
                    BFToken::PtrLeft => $left,
                    BFToken::PtrRight => $right,
                    BFToken::Print => $print,
                    BFToken::Input => $input,
                    BFToken::JumpForwards => $forwards,
                    BFToken::JumpBackwards => $backwards,
                    _ => return Err(TokenParseError),
                }));
            }
        }
    };
}

pub(crate) use multi_char_tokenizer;
pub(crate) use single_char_tokenizer;
pub(crate) use token_to_string;
