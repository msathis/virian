use nom::*;
use nom::digit;
use nom::types::CompleteStr;

use crate::assembler::Token;

/// Parser for integer numbers, which we preface with `#` in our assembly language:
/// #100
named!(pub register<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_num: digit >>
            (
                Token::register(reg_num.parse::<u8>().unwrap())
            )
        )
    )
);

#[test]
fn test_parse_integer_operand() {
    // Test a valid integer operand
    let result = register(CompleteStr("$0"));
    assert_eq!(result.is_ok(), true);
    let (rest, value) = result.unwrap();
    assert_eq!(rest, CompleteStr(""));
    assert_eq!(value, Token::Register { reg_num: 0 });

    // Test an invalid one (missing the #)
    let result = register(CompleteStr("10"));
    assert_eq!(result.is_ok(), false);
}