use std::collections::VecDeque;
use std::io::Write;
use crate::trace::Trace;

use super::{ Instruction, Cpu, memory::Memory };

pub const ADD_CYCLES: usize = 2;
pub const SUB_CYCLES: usize = 2;
pub const MUL_CYCLES: usize = 12;
pub const DIV_CYCLES: usize = 24;
pub const LOAD_CYCLES: usize = 2;
pub const JUMP_CYCLES: usize = 1;

/// 单周期执行的 CPU
pub struct SingleCycleCpu<'a> {
    pub(crate) regs: [i32;32],
    pub(crate) instruction_queue: VecDeque<Instruction>,
    pub(crate) memory: Memory,
    pub(crate) trace: &'a mut Trace,
    pub(crate) cycles: usize
}

impl<'a> Cpu for SingleCycleCpu<'a> {
    fn run(&mut self) {
        println!("Start execute instructions!");
        loop {
            if let Some(inst) = self.instruction_queue.pop_front() {
                match inst {
                    Instruction::Add(operand) => { 
                        self.regs[operand.target as usize] = if let Some(add_res) = self.regs[operand.operand1 as usize].checked_add(self.regs[operand.operand2 as usize]){ add_res }else { 0 };
                        self.cycles += ADD_CYCLES; 
                    },
                    Instruction::Sub(operand) => { 
                        self.regs[operand.target as usize] = if let Some(sub_res) = self.regs[operand.operand1 as usize].checked_sub(self.regs[operand.operand2 as usize]){ sub_res }else { 0 };
                        self.cycles += SUB_CYCLES; 
                    },
                    Instruction::Mul(operand) => { 
                        self.regs[operand.target as usize] = if let Some(mul_res) = self.regs[operand.operand1 as usize].checked_mul(self.regs[operand.operand2 as usize]){ mul_res }else { 0 };
                        self.cycles += MUL_CYCLES; 
                    },
                    Instruction::Div(operand) => { 
                        self.regs[operand.target as usize] = if let Some(div_res) = self.regs[operand.operand1 as usize].checked_div(self.regs[operand.operand2 as usize]){ div_res }else { 0 };
                        self.cycles += DIV_CYCLES; 
                    },
                    Instruction::Ld(reg1, reg2, imm) => {
                        let addr = (self.regs[reg2] + (imm as i32)) as u32;
                        let val = self.memory.read(addr);
                        self.regs[reg1] = val;
                        self.cycles += LOAD_CYCLES;
                    },
                    Instruction::Sd(reg1, reg2, imm) => {
                        let addr = (self.regs[reg2] + (imm as i32)) as u32;
                        self.memory.write(addr, self.regs[reg1]);
                    },

                    Instruction::Jump(_, _) => {
                        self.cycles += JUMP_CYCLES;
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
        println!("[Debug] cycles: {}", self.cycles);
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
            trace: trace,
            cycles: 0
        }
    }

    pub fn set_regs(&mut self, index: usize, number: i32) {
        self.regs[index] = number;
    }

}