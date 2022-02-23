/// CPU 的 Trait
pub trait CpuExecute{
    fn execute(&self);
}

/// 操作数
pub struct Operand{
    target: usize,
    operand1: usize,
    operand2: usize
}
/// 指令类型
pub enum Instruction{
    Add(Operand)
}

