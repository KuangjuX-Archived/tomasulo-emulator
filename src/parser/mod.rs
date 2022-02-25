use regex::Regex;

use super::cpu::{ Instruction, Operand };

pub fn parse<S>(inst: S) -> Instruction 
    where S: Into<String>
{
    let inst : String = inst.into();
    match inst.chars().nth(0) {
        Some('L') => {
            let pattern = Regex::new(r"LD,R([0-9]*),0x([0-9a-fA-F]+)").unwrap();
            let cap = pattern.captures(&inst).unwrap();
            Instruction::Ld(
                Operand::new(
                    cap[1].parse::<usize>().unwrap(), 
                    usize::from_str_radix(&cap[2], 16).unwrap(), 
                    0
                )
            )
        },

        Some('A') => {
            let pattern = Regex::new(r"ADD,R([0-9]*),R([0-9]*),R([0-9]*)").unwrap();
            let cap = pattern.captures(&inst).unwrap();
            Instruction::Add(
                Operand::new(
                    usize::from_str_radix(&cap[1], 10).unwrap(),
                    usize::from_str_radix(&cap[2], 10).unwrap(), 
                    usize::from_str_radix(&cap[3], 10).unwrap()
                )
            )
        },

        Some('S') => {
            let pattern = Regex::new(r"SUB,R([0-9]*),R([0-9]*),R([0-9]*)").unwrap();
            let cap = pattern.captures(&inst).unwrap();
            Instruction::Sub(
                Operand::new(
                    usize::from_str_radix(&cap[1], 10).unwrap(),
                    usize::from_str_radix(&cap[2], 10).unwrap(), 
                    usize::from_str_radix(&cap[3], 10).unwrap()
                )
            )
        },

        Some('M') => {
            let pattern = Regex::new(r"MUL,R([0-9]*),R([0-9]*),R([0-9]*)").unwrap();
            let cap = pattern.captures(&inst).unwrap();
            Instruction::Mul(
                Operand::new(
                    usize::from_str_radix(&cap[1], 10).unwrap(),
                    usize::from_str_radix(&cap[2], 10).unwrap(), 
                    usize::from_str_radix(&cap[3], 10).unwrap()
                )
            )
        }

        Some('D') => {
            let pattern = Regex::new(r"DIV,R([0-9]*),R([0-9]*),R([0-9]*)").unwrap();
            let cap = pattern.captures(&inst).unwrap();
            Instruction::Div(
                Operand::new(
                    usize::from_str_radix(&cap[1], 10).unwrap(),
                    usize::from_str_radix(&cap[2], 10).unwrap(), 
                    usize::from_str_radix(&cap[3], 10).unwrap()
                )
            )
        }

        _ => {
            Instruction::Invalid
        }
    }
}
