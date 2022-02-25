use std::fs::File;
use std::collections::VecDeque;
use std::io::Read;
use super::{ Instruction, Cpu};
use crate::parser::parse;

/// 单周期执行的 CPU
pub struct SingleCycleCPU {
    pub(crate) regs: [usize;32],
    pub(crate) instruction_queue: VecDeque<Instruction>
}

impl Cpu for SingleCycleCPU {
    fn execute(&mut self) {
        println!("Start execute instructions!");
        loop {
            if let Some(inst) = self.instruction_queue.pop_back() {
                match inst {
                    Instruction::Ld(operand) => {
                        self.regs[operand.target] = operand.operand1;
                    },

                    Instruction::Add(operand) => {
                        self.regs[operand.target] = self.regs[operand.operand1] + self.regs[operand.operand2];
                    },

                    Instruction::Sub(operand) => {

                    }

                    _ => {

                    }
                }
            }
        }
    }
}

impl SingleCycleCPU {
    pub fn new() -> Self {
        Self{
            regs: [0usize;32],
            instruction_queue: VecDeque::new()
        }
    }

    /// 读取指令
    pub fn read_inst(&mut self, filename: String) -> Result<(), String>{
        let mut file = File::open(filename).map_err( |err| { format!("err: {}", err) })?;
        let mut insts: String = String::new();
        file.read_to_string(&mut insts).map_err(|err| { format!("err: {}", err) })?;
        for inst in insts.lines() {
            let inst = parse(inst);
            println!("inst: {:?}", inst);
            self.instruction_queue.push_back(inst);
        }
        Ok(())
    }
}