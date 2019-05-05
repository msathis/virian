use crate::instructions::Opcode;

#[derive(Debug)]
pub struct VM {
    //Array of registers simulating hardware registers
    pub registers: [i32; 32],

    //Program counter tracks current program instruction byte executing
    pc: usize,

    //Bytes of the program
    program: Vec<u8>,

    //Remainder value
    remainder: u32,

    //Equality check result of the last operation
    equal: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal: false,
        }
    }

    /// Loops as long as instructions can be executed.
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) -> bool {
        self.execute_instruction()
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let target = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += target;
            }
            Opcode::JMPB => {
                let target = self.registers[self.next_8_bits() as usize] as usize;
                self.pc -= target;
            }
            Opcode::EQ => {
                let reg1 = self.registers[self.next_8_bits() as usize] as usize;
                let reg2 = self.registers[self.next_8_bits() as usize] as usize;

                self.equal = reg1 == reg2;
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let reg1 = self.registers[self.next_8_bits() as usize] as usize;
                let reg2 = self.registers[self.next_8_bits() as usize] as usize;

                self.equal = reg1 != reg2;
                self.next_8_bits();
            }
            Opcode::LT => {
                let reg1 = self.registers[self.next_8_bits() as usize] as usize;
                let reg2 = self.registers[self.next_8_bits() as usize] as usize;

                self.equal = reg1 < reg2;
                self.next_8_bits();
            }
            Opcode::LTQ => {
                let reg1 = self.registers[self.next_8_bits() as usize] as usize;
                let reg2 = self.registers[self.next_8_bits() as usize] as usize;

                self.equal = reg1 <= reg2;
                self.next_8_bits();
            }
            Opcode::GT => {
                let reg1 = self.registers[self.next_8_bits() as usize] as usize;
                let reg2 = self.registers[self.next_8_bits() as usize] as usize;

                self.equal = reg1 > reg2;
                self.next_8_bits();
            }
            Opcode::GTQ => {
                let reg1 = self.registers[self.next_8_bits() as usize] as usize;
                let reg2 = self.registers[self.next_8_bits() as usize] as usize;

                self.equal = reg1 >= reg2;
                self.next_8_bits();
            }
            Opcode::JEQ => {
                let target = self.registers[self.next_8_bits() as usize] as usize;

                if self.equal {
                    self.pc = target;
                }
            }
            Opcode::JNEQ => {
                let target = self.registers[self.next_8_bits() as usize] as usize;

                if !self.equal {
                    self.pc = target;
                }
            }
            _ => {
                println!("Invalid code encountered");
                return true;
            }
        }
        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![6, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 244]; // Remember, this is how we represent 500 using two u8s in little endian format
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![5, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = VM::new();
        test_vm.pc = 8;
        test_vm.registers[0] = 4;
        test_vm.program = vec![0, 0, 0, 4, 0, 1, 0, 2, 8, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 6);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal, false);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.equal = true;
        test_vm.program = vec![15, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }
}
