use regex::Regex;
use std::fs::File;
use std::io::Read;

use super::cpu::{ Instruction, Operand, Cpu };


pub struct Parser ();

impl Parser {
    pub const fn new() -> Self {
        Self()
    }

    pub fn parse<S>(&self, inst: S) -> Instruction 
        where S: Into<String>
    {
        let inst : String = inst.into();
        match inst.chars().nth(0) {
            Some('L') => {
                let pattern = Regex::new(r"LD,R([0-9]*),0x([0-9a-fA-F]+)").unwrap();
                let cap = pattern.captures(&inst).unwrap();
                Instruction::Ld(
                    Operand::new(
                        isize::from_str_radix(&cap[1], 10).unwrap(), 
                        isize::from_str_radix(&cap[2], 16).unwrap(), 
                        0
                    )
                )
            },

            Some('A') => {
                let pattern = Regex::new(r"ADD,R([0-9]*),R([0-9]*),R([0-9]*)").unwrap();
                let cap = pattern.captures(&inst).unwrap();
                Instruction::Add(
                    Operand::new(
                        isize::from_str_radix(&cap[1], 10).unwrap(),
                        isize::from_str_radix(&cap[2], 10).unwrap(), 
                        isize::from_str_radix(&cap[3], 10).unwrap()
                    )
                )
            },

            Some('S') => {
                let pattern = Regex::new(r"SUB,R([0-9]*),R([0-9]*),R([0-9]*)").unwrap();
                let cap = pattern.captures(&inst).unwrap();
                Instruction::Sub(
                    Operand::new(
                        isize::from_str_radix(&cap[1], 10).unwrap(),
                        isize::from_str_radix(&cap[2], 10).unwrap(), 
                        isize::from_str_radix(&cap[3], 10).unwrap()
                    )
                )
            },

            Some('M') => {
                let pattern = Regex::new(r"MUL,R([0-9]*),R([0-9]*),R([0-9]*)").unwrap();
                let cap = pattern.captures(&inst).unwrap();
                Instruction::Mul(
                    Operand::new(
                        isize::from_str_radix(&cap[1], 10).unwrap(),
                        isize::from_str_radix(&cap[2], 10).unwrap(), 
                        isize::from_str_radix(&cap[3], 10).unwrap()
                    )
                )
            }

            Some('D') => {
                let pattern = Regex::new(r"DIV,R([0-9]*),R([0-9]*),R([0-9]*)").unwrap();
                let cap = pattern.captures(&inst).unwrap();
                Instruction::Div(
                    Operand::new(
                        isize::from_str_radix(&cap[1], 10).unwrap(),
                        isize::from_str_radix(&cap[2], 10).unwrap(), 
                        isize::from_str_radix(&cap[3], 10).unwrap()
                    )
                )
            }

            _ => {
                Instruction::Invalid
            }
        }
    }

    pub fn read_inst<C, S>(&self, cpu: &mut C, filename: S) -> Result<(), String>
        where C: Cpu, S: Into<String> 
    {
        let mut file = File::open(filename.into()).map_err( |err| { format!("err: {}", err) })?;
        let mut insts: String = String::new();
        file.read_to_string(&mut insts).map_err(|err| { format!("err: {}", err) })?;
        for inst in insts.lines() {
            let inst = self.parse(inst);
            println!("inst: {:?}", inst);
            cpu.add_inst(inst);
        }
        Ok(())
    }
}
