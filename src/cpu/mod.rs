pub mod common;
pub mod tomasulo;
pub mod memory;
pub use common::SingleCycleCpu;
pub use tomasulo::{ TomasuloCpu, ResStationType };
use memory::Memory;

/// CPU 的 Trait
pub trait Cpu{
    fn run(&mut self);
    fn add_inst(&mut self, inst: Instruction);
    fn trace<S>(&mut self, s: S) where S: Into<String>;
    fn write_memory(&mut self, addr: u32, val: i32);
}

/// 操作数
#[derive(Debug, Clone, Copy)]
pub struct Operand {
    target: usize,
    operand1: usize,
    operand2: usize
}

impl Operand {
    pub fn new(target: usize, operand1: usize, operand2: usize) -> Self {
        Self{
            target,
            operand1,
            operand2
        }
    }
}
/// 指令类型
#[derive(Debug, Clone, Copy)]
pub enum Instruction{
    Add(Operand),
    Sub(Operand),
    Mul(Operand),
    Div(Operand),
    Ld(usize, usize, u32),
    Sd(usize, usize, u32),
    /// 这里的 Jump 并不做实际跳转，只是
    /// 当做没有目标寄存器的指令
    Jump(usize, usize),
    Invalid
}


