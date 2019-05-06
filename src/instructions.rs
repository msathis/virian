use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub enum Opcode {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    LT,
    GT,
    LTQ,
    GTQ,
    JEQ,
    JNEQ,
    HLT,
    IGL,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::LOAD,
            1 => return Opcode::ADD,
            2 => return Opcode::SUB,
            3 => return Opcode::MUL,
            4 => return Opcode::DIV,
            5 => return Opcode::JMP,
            6 => return Opcode::HLT,
            7 => return Opcode::JMPF,
            8 => return Opcode::JMPB,
            9 => return Opcode::EQ,
            10 => return Opcode::NEQ,
            11 => return Opcode::LT,
            12 => return Opcode::LTQ,
            13 => return Opcode::GT,
            14 => return Opcode::GTQ,
            15 => return Opcode::JEQ,
            16 => return Opcode::JNEQ,
            _ => return Opcode::IGL,
        }
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        let lowercased_opcode = v.to_lowercase();

        match CompleteStr(&lowercased_opcode) {
            CompleteStr("load") => Opcode::LOAD,
            CompleteStr("add") => Opcode::ADD,
            CompleteStr("sub") => Opcode::SUB,
            CompleteStr("mul") => Opcode::MUL,
            CompleteStr("div") => Opcode::DIV,
            CompleteStr("hlt") => Opcode::HLT,
            CompleteStr("jmp") => Opcode::JMP,
            CompleteStr("jmpf") => Opcode::JMPF,
            CompleteStr("jmpb") => Opcode::JMPB,
            CompleteStr("eq") => Opcode::EQ,
            CompleteStr("neq") => Opcode::NEQ,
            CompleteStr("gte") => Opcode::GTQ,
            CompleteStr("gt") => Opcode::GT,
            CompleteStr("lte") => Opcode::LTQ,
            CompleteStr("lt") => Opcode::LT,
            CompleteStr("jmpe") => Opcode::JEQ,
            _ => Opcode::IGL,
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
