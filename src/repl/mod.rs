use std::io;
use std::num::ParseIntError;

use nom::types::CompleteStr;

use crate::assembler::program_parser::program;
use crate::VM;

#[derive(Debug)]
pub struct REPL {
    vm: VM,
    commands_buffer: Vec<String>,
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            vm: VM::new(),
            commands_buffer: vec![],
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        println!("Welcome to virian. Enter your command.");
        let mut buffer = String::new();

        loop {
            println!(">>>");
            buffer.clear();
            io::stdin().read_line(&mut buffer)?;
            let buffer = buffer.trim();
            self.commands_buffer.push(buffer.to_string());

            match buffer.as_ref() {
                "history" => {
                    for item in &self.commands_buffer {
                        println!("{}", &item);
                    }
                }
                "registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing")
                }
                "quit" | "q" => {
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
                }
                _ => {
                    let (_, results) = program(CompleteStr(buffer)).unwrap();
                    let bytes = results.to_bytes();
                    for byte in bytes {
                        self.vm.add_byte(byte)
                    }
                    self.vm.run_once();
                }
            }
        }
    }
}
