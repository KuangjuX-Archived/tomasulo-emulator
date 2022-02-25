
pub mod common;
pub mod tomasulo;
/// CPU 的 Trait
pub trait Cpu{
    fn execute(&mut self);
}

/// 操作数
#[derive(Debug)]
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
#[derive(Debug)]
pub enum Instruction{
    Add(Operand),
    Sub(Operand),
    Mul(Operand),
    Div(Operand),
    Ld(Operand),
    Invalid
}

