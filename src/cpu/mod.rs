
pub mod common;
pub mod tomasulo;
pub use common::SingleCycleCpu;
pub use tomasulo::TomasuloCpu;
/// CPU 的 Trait
pub trait Cpu{
    fn execute(&mut self);
}

/// 操作数
#[derive(Debug)]
pub struct Operand {
    target: isize,
    operand1: isize,
    operand2: isize
}

impl Operand {
    pub fn new(target: isize, operand1: isize, operand2: isize) -> Self {
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

