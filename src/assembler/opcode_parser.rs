use nom::*;
use nom::types::CompleteStr;

use crate::assembler::Token;
use crate::instructions::Opcode;

named!(pub opcode<CompleteStr, Token>,
  do_parse!(
      opcode: alpha1 >>
      (
        {
            Token::opcode(Opcode::from(opcode))
        }
      )
  )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = opcode(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().1, Token::Op { code: Opcode::LOAD });

        let result = opcode(CompleteStr(""));
        assert_eq!(result.is_ok(), false);

        let result = opcode(CompleteStr("$a"));
        assert_eq!(result.is_ok(), false);

        let result = opcode(CompleteStr("0"));
        assert_eq!(result.is_ok(), false);
    }
}