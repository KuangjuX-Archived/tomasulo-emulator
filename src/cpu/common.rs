use std::fs::File;
use std::collections::VecDeque;
use std::io::Read;
use super::Instruction;

/// 单周期执行的 CPU
pub struct SingleCycleCPU {
    pub(crate) regs: [u32;32],
    pub(crate) instruction_queue: VecDeque<Instruction>
}

impl SingleCycleCPU {
    pub fn new() -> Self {
        Self{
            regs: [0u32;32],
            instruction_queue: VecDeque::new()
        }
    }

    /// 读取指令
    pub fn read_inst(&mut self, filename: String) -> Result<(), String>{
        let mut file = File::open(filename).map_err( |err| { format!("err: {}", err) })?;
        let mut insts: String = String::new();
        file.read_to_string(&mut insts).map_err(|err| { format!("err: {}", err) })?;
        for inst in insts.lines() {
            
        }
        Ok(())
    }
}