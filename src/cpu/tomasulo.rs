use std::collections::VecDeque;

use super::{Instruction, Operand};
pub struct TomasuloCpu {
    /// 寄存器文件
    regs: [isize;32],
    /// 指令队列
    instruction_queue: VecDeque<Instruction>,
    /// 保留站
    rs: ReservedStation,
    /// ROB
    rob: ReorderBuffer
}



/// 保留站
pub struct ReservedStation {
    /// 是否被占用
    busy: bool,
    /// 内存地址，仅 store 和 load 使用
    address: Option<usize>,
    /// 指令类型
    operand: Operand,
    /// Qj
    rs_index_1: Option<usize>,
    /// Qk
    rs_index_2: Option<usize>,
    /// Vj,
    value_1: Option<usize>,
    /// Vk
    value_2: Option<usize>
}

pub struct ReorderBuffer {
}