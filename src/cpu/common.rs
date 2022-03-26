use std::collections::VecDeque;
use std::io::Write;
use crate::trace::Trace;

use super::{ Instruction, Cpu, memory::Memory };

/// 单周期执行的 CPU
pub struct SingleCycleCpu<'a> {
    pub(crate) regs: [i32;32],
    pub(crate) instruction_queue: VecDeque<Instruction>,
    pub(crate) memory: Memory,
    pub(crate) trace: &'a mut Trace
}

impl<'a> Cpu for SingleCycleCpu<'a> {
    fn run(&mut self) {
        println!("Start execute instructions!");
        loop {
            if let Some(inst) = self.instruction_queue.pop_front() {
                match inst {
                    Instruction::Add(operand) => { self.regs[operand.target as usize] = self.regs[operand.operand1 as usize] + self.regs[operand.operand2 as usize]; },
                    Instruction::Sub(operand) => { self.regs[operand.target as usize] = self.regs[operand.operand1 as usize] - self.regs[operand.operand2 as usize]; },
                    Instruction::Mul(operand) => { self.regs[operand.target as usize] = self.regs[operand.operand1 as usize] * self.regs[operand.operand2 as usize]; },
                    Instruction::Div(operand) => { self.regs[operand.target as usize] = self.regs[operand.operand1 as usize] / self.regs[operand.operand2 as usize]; },
                    Instruction::Ld(reg1, reg2, imm) => {
                        let addr = (self.regs[reg2] + (imm as i32)) as u32;
                        let val = self.memory.read(addr);
                        self.regs[reg1] = val;
                    },
                    Instruction::Sd(reg1, reg2, imm) => {
                        let addr = (self.regs[reg2] + (imm as i32)) as u32;
                        self.memory.write(addr, self.regs[reg1]);
                    }
                    _ => {}
                }
            }else{ break; }
            let mut info: String = String::new();
            for (index, reg) in self.regs.iter().enumerate() {
                info.push_str(format!("reg{}: {}; ", index, reg).as_str());
            }
            self.trace(info);
        }
        println!("Finish execute!");
    }

    fn add_inst(&mut self, inst: Instruction) {
        self.instruction_queue.push_back(inst);
    }

    fn trace<S>(&mut self, s: S)
        where S: Into<String> 
    {
        let s: String = s.into();
        writeln!(self.trace.file, "{}", s).unwrap();
    }

    fn write_memory(&mut self, addr: u32, val: i32) {
        self.memory.write(addr, val);
    }

}

impl<'a> SingleCycleCpu<'a> {
    pub fn new(trace: &'a mut Trace) -> Self {
        Self{
            regs: [0i32;32],
            instruction_queue: VecDeque::new(),
            memory: Memory::init(),
            trace: trace
        }
    }

    pub fn set_regs(&mut self, index: usize, number: i32) {
        self.regs[index] = number;
    }

}