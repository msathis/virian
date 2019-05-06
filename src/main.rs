use crate::repl::REPL;
use crate::vm::VM;

pub mod assembler;
pub mod instructions;
pub mod repl;
pub mod vm;

fn main() {
    let mut repl = REPL::new();
    repl.run().unwrap();
}
