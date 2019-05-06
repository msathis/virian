use crate::repl::REPL;
use crate::vm::VM;

pub mod instructions;
pub mod repl;
pub mod vm;
pub mod assembler;

fn main() {
    let mut repl = REPL::new();
    repl.run().unwrap();
}
