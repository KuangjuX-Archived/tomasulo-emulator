use std::collections::VecDeque;
use crate::trace::Trace;

use super::{ Instruction, Cpu };

/// 单周期执行的 CPU
pub struct SingleCycleCpu<'a> {
    pub(crate) regs: [isize;32],
    pub(crate) instruction_queue: VecDeque<Instruction>,
    pub(crate) trace: &'a mut Trace
}

impl<'a> Cpu for SingleCycleCpu<'a> {
    fn run(&mut self) {
        println!("Start execute instructions!");
        loop {
            if let Some(inst) = self.instruction_queue.pop_front() {
                match inst {
                    Instruction::Ld(operand) => { self.regs[operand.target as usize] = operand.operand1; },
                    Instruction::Add(operand) => { self.regs[operand.target as usize] = self.regs[operand.operand1 as usize] + self.regs[operand.operand2 as usize]; },
                    Instruction::Sub(operand) => { self.regs[operand.target as usize] = self.regs[operand.operand1 as usize] - self.regs[operand.operand2 as usize]; },
                    Instruction::Mul(operand) => { self.regs[operand.target as usize] = self.regs[operand.operand1 as usize] * self.regs[operand.operand2 as usize]; },
                    Instruction::Div(operand) => { self.regs[operand.target as usize] = self.regs[operand.operand1 as usize] / self.regs[operand.operand2 as usize]; }
                    _ => {}
                }
            }else{ break; }
        }
        println!("Finish execute!");
    }

    fn add_inst(&mut self, inst: Instruction) {
        self.instruction_queue.push_back(inst);
    }

    fn trace<S>(&mut self, s: S) {

    }
}

impl<'a> SingleCycleCpu<'a> {
    pub fn new(trace: &'a mut Trace) -> Self {
        Self{
            regs: [0isize;32],
            instruction_queue: VecDeque::new(),
            trace: trace
        }
    }
}