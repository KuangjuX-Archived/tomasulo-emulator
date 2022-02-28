use std::collections::VecDeque;
use std::convert::From;
use super::{ Instruction, Operand, Cpu };

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum ResStationType {
    AddSub,
    MulDiv,
    LoadBuffer
}

impl From<Instruction> for ResStationType {
    fn from(item: Instruction) -> ResStationType {
        match item {
            Instruction::Add(_) | Instruction::Sub(_) => { ResStationType::AddSub },
            Instruction::Mul(_) | Instruction::Div(_) => { ResStationType::MulDiv },
            Instruction::Ld(_) => { ResStationType::LoadBuffer },
            _ => { panic!("[Error] Invalid instruction") }
        }
    }
}

/// 保留站
pub struct ReservedStation {
    /// 保留站类型
    rs_type: ResStationType,
    /// 是否被占用
    busy: bool,
    /// 保留站内部信息
    inner: ResStationInner
}

pub struct ResStationInner {
    /// 内存地址，仅 store 和 load 使用
    address: Option<usize>,
    /// 指令类型
    operand: Option<Operand>,
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
    inner: ROBInner
}

pub struct ROBInner {   
    inst: Option<Instruction>,
    dest: Option<usize>,
    value: Option<usize>
}

/// 寄存器状态
#[derive(Clone, Copy)]
pub struct RegisterStatus {
    busy: bool,
    reorder: Option<usize>
}

pub struct TomasuloCpu {
    /// 周期数
    cycles: usize,
    /// 寄存器状态
    reg_stat: Vec<RegisterStatus>,
    /// 寄存器文件
    regs: [isize;32],
    /// 指令队列
    instruction_queue: VecDeque<Instruction>,
    /// 保留站
    rs: Vec<ReservedStation>,
    /// ROB
    rob: Vec<ReorderBuffer>
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
            cycles: 0,
            reg_stat: vec![RegisterStatus{ busy: false, reorder: None }; 32],
            regs: [0isize;32],
            instruction_queue: VecDeque::new(),
            rs: vec![],
            rob: vec![]
        };
        // 为 CPU 添加保留站
        cpu.add_rs(ResStationType::AddSub, 3);
        cpu.add_rs(ResStationType::MulDiv, 2);
        cpu.add_rs(ResStationType::LoadBuffer, 2);
        // 为 CPU 添加 ROB
        cpu.add_rob(6);
        cpu
    }

    /// 添加保留站
    fn add_rs(&mut self, rs_type: ResStationType, count: usize) {
        for _ in 0..count {
            self.rs.push(
                ReservedStation { 
                    rs_type: rs_type,
                    busy: false,
                    inner: ResStationInner{
                        address: None,
                        operand: None,
                        rs_index: None,
                        rs_value: None,
                        rt_index: None,
                        rt_value: None
                    }
                }
            )
        }
    }

    /// 添加 ROB
    fn add_rob(&mut self, count: usize) {
        for _ in 0..count {
            self.rob.push(ReorderBuffer {
                busy: false,
                ready: false,
                inner: ROBInner {
                    inst: None,
                    dest: None,
                    value: None
                }
            })
        }
    } 

    fn update_op(&mut self, reg_index: usize, rs: usize, rob: usize) {
        let reg_stat = &mut self.reg_stat;
        let rs = &mut self.rs[rs];
        let rob = &mut self.rob[rob];
        // 如果操作数的目前的状态是 busy 表示当前操作数不在寄存器中
        // 而将要被前面的指令写回或者在 ROB 中
        if reg_stat[reg_index].busy  {
            let h = reg_stat[reg_index].reorder.unwrap();
            if self.rob[h].ready {
                // 在 ROB 中已经将该寄存器的值计算完成，但仍然没有 commit
                // 此时直接进行赋值即可
                rs.inner.rs_value = Some(self.rob[h].inner.value.unwrap());
                rs.inner.rs_index = None;
            }else{
                // 此时在 ROB 仍然没有计算完, 记录为 ROB 的序号
                rs.inner.rs_index = Some(h);
            }
        }else{
            // 目前操作数在寄存器堆中
            rs.inner.rs_value = Some(reg_stat[reg_index].reorder).unwrap();
            rs.inner.rs_index = None;
        }
    }

    /// 查看是否能发射
    fn can_issue(&self, rs_type: ResStationType) -> Option<(usize, usize)> {
        for i in 0..self.rs.len() {
            if !self.rs[i].busy && self.rs[i].rs_type == rs_type {
                for j in 0..self.rob.len() {
                    if !self.rob[j].busy {
                        return Some((i, j))
                    }
                }
                return None
            }
        }
        None
    }

    /// 发射指令，每周期发射一条指令
    pub(crate) fn issue(&mut self) {
        let inst = self.instruction_queue.pop_front().unwrap();
        let rs_type: ResStationType = inst.into();
        if let Some((rs, rob)) = self.can_issue(rs_type) {
            match rs_type {
                // 浮点数运算操作
                ResStationType::AddSub | ResStationType::MulDiv => {
                    match inst {
                        Instruction::Add(op) | Instruction::Sub(op) | Instruction::Mul(op) | Instruction::Div(op) => {
                            // let rs = &mut self.rs[rs];
                            // let rob = &mut self.rob[rob];
                            let r1 = op.operand1 as usize;
                            let r2 = op.operand2 as usize;
                            self.update_op(r1, rs, rob);
                            self.update_op(r2, rs, rob)
                        },

                        _ => {}
                    }
                },
                ResStationType::LoadBuffer => {

                }
            }
        }
    }

    /// 执行指令
    pub(crate) fn exec(&mut self) {

    }

    /// 提交指令
    pub(crate) fn commit(&mut self) {

    }
}