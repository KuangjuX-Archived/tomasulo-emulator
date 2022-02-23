
pub mod common;
pub mod tomasulo;
/// CPU 的 Trait
pub trait CpuExecute{
    fn execute(&self, instruction: Instruction);
}

/// 操作数
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
pub enum Instruction{
    Add(Operand),
    Sub(Operand),
    Mul(Operand),
    Div(Operand),
    Ld(Operand)
}

