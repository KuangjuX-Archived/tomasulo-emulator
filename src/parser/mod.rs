use std::ops::Index;

use super::cpu::{ Instruction, Operand };

pub fn parse<S>(inst: S) -> Instruction 
    where S: Into<String>
{
    let inst : String = inst.into();
    match inst.chars().nth(0) {
        Some('A') => {
            
        },

        _ => {

        }
    }
}
