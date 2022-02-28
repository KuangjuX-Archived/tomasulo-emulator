use std::collections::VecDeque;

use super::{ Instruction, Operand, Cpu };
pub struct TomasuloCpu {
    /// 寄存器文件
    regs: [isize;32],
    /// 指令队列
    instruction_queue: VecDeque<Instruction>,
    /// 保留站
    rs: VecDeque<ReservedStation>,
    /// ROB
    rob: VecDeque<ReorderBuffer>
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum ResStationType {
    AddSub,
    MulDiv,
    LoadBuffer
}

/// 保留站
pub struct ReservedStation {
    /// 保留站类型
    rs_type: ResStationType,
    /// 是否被占用
    busy: bool,
    /// 保留站内部信息
    inner: Option<ResStationInner>
}

pub struct ResStationInner {
    /// 内存地址，仅 store 和 load 使用
    address: Option<usize>,
    /// 指令类型
    operand: Operand,
    /// Qj
    rs_index: Option<usize>,
    /// Qk
    rt_index: Option<usize>,
    /// Vj,
    rs_value: Option<usize>,
    /// Vk
    rt_value: Option<usize>
}

/// ROB
pub struct ReorderBuffer {
    busy: bool,
    ready: bool,
    inner: Option<ROBInner>
}

pub struct ROBInner {   
    inst: Instruction,
    dest: usize,
    value: usize
}

impl Cpu for TomasuloCpu {
    fn add_inst(&mut self, inst: Instruction) {
        self.instruction_queue.push_back(inst);
    }

    fn execute(&mut self) {
        todo!()
    }
}

impl TomasuloCpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            regs: [0isize;32],
            instruction_queue: VecDeque::new(),
            rs: VecDeque::new(),
            rob: VecDeque::new()
        };
        // 为 CPU 添加保留站
        cpu.add_rs(ResStationType::AddSub, 3);
        cpu.add_rs(ResStationType::MulDiv, 2);
        cpu.add_rs(ResStationType::LoadBuffer, 2);
        // 为 CPU 添加 ROB
        cpu.add_rob(6);
        cpu
    }

    fn add_rs(&mut self, rs_type: ResStationType, count: usize) {
        for _ in 0..count {
            self.rs.push_back(
                ReservedStation { 
                    rs_type: rs_type,
                    busy: false,
                    inner: None
                }
            )
        }
    }

    fn add_rob(&mut self, count: usize) {
        for _ in 0..count {
            self.rob.push_back(ReorderBuffer {
                busy: false,
                ready: false,
                inner: None
            })
        }
    } 

    /// 查看是否能发射
    pub(crate) fn can_issue(&mut self, rs_type: ResStationType) -> Option<(&mut ReservedStation, &mut ReorderBuffer)> {
        for rs in self.rs.iter_mut() {
            if rs.busy == false && rs.rs_type == rs_type {
                for rob in self.rob.iter_mut() {
                    if rob.busy == false {
                        return Some((rs, rob))
                    }
                }
                return None
            }
        }
        None
    }
    /// 发射指令，目前单周期内发射两条指令
    pub(crate) fn issue(&mut self) {
        let inst = self.instruction_queue.pop_front().unwrap();
        if let Some(rs, rob) = self.can_issue();
    }

    /// 执行指令
    pub(crate) fn exec(&mut self) {

    }

    /// 提交指令
    pub(crate) fn commit(&mut self) {

    }
}