use crate::instructions::Opcode;

pub mod opcode_parser;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
}

impl Token {
    pub fn new(code: Opcode) -> Self {
        Token::Op { code }
    }
}