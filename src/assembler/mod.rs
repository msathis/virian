use crate::instructions::Opcode;

pub mod opcode_parser;
pub mod operand_parser;
pub mod register_parser;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
}

impl Token {
    pub fn opcode(code: Opcode) -> Self {
        Token::Op { code }
    }

    pub fn operand(value: i32) -> Self {
        Token::IntegerOperand { value }
    }

    pub fn register(reg_num: u8) -> Self {
        Token::Register { reg_num }
    }
}