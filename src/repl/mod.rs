use std::io;
use std::num::ParseIntError;

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
        println!("Enter your command. Type .help for help");
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
                    let results = self.parse_hex(buffer.as_ref());
                    match results {
                        Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte)
                            }
                        }
                        Err(_e) => println!(
                            "Unable to decode hex string. Please enter 4 groups of 2 hex characters."
                        ),
                    };
                    self.vm.run_once();
                }
            }
        }
    }

    fn parse_hex(&self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
}
