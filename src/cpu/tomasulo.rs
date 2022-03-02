use std::collections::VecDeque;
use std::convert::From;
use super::{ Instruction, Cpu };

pub const ADD_CYCLES: usize = 1;
pub const SUB_CYCLES: usize = 1;
pub const MUL_CYCLES: usize = 6;
pub const DIV_CYCLES: usize = 12;

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
    inst: Option<Instruction>,
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
#[derive(Debug)]
pub struct ReorderBuffer {
    busy: bool,
    ready: bool,
    inner: ROBInner
}


#[derive(Debug)]
pub struct ROBInner {   
    inst: Option<Instruction>,
    dest: Option<usize>,
    value: Option<usize>
}

/// 寄存器状态
#[derive(Clone, Copy)]
pub struct RegisterStatus {
    busy: bool,
    /// reorder 不应该标识为数组的索引
    /// 应该标示为独一无二的数字，或许
    /// 指针是个不错的选择
    reorder: Option<usize>
}

pub struct ExecUint {
    busy: bool,
    rs_type: ResStationType,
    cycles: usize,
    /// 保留站的索引
    rs_index: usize
}

pub struct TomasuloCpu {
    /// 是否完成
    done: bool,
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
    rob: Vec<ReorderBuffer>,
    /// 执行单元
    exec_units: Vec<ExecUint>
}


impl Cpu for TomasuloCpu {
    fn add_inst(&mut self, inst: Instruction) {
        self.instruction_queue.push_back(inst);
    }

    fn run(&mut self) {
        loop {
            if !self.done(){ self.single_cycle(); }
            else { break; }
        }
        println!("[Debug] Cpu run finished");
    }
}

impl TomasuloCpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            done: false,
            cycles: 0,
            reg_stat: vec![RegisterStatus{ busy: false, reorder: None }; 32],
            regs: [0isize;32],
            instruction_queue: VecDeque::new(),
            rs: vec![],
            rob: vec![],
            exec_units: vec![]
        };
        // 为 CPU 添加保留站
        cpu.add_rs(ResStationType::AddSub, 3);
        cpu.add_rs(ResStationType::MulDiv, 2);
        cpu.add_rs(ResStationType::LoadBuffer, 2);
        // 为 CPU 添加 ROB
        cpu.add_rob(6);
        // 为 CPU 添加执行单元
        cpu.add_exec_unit(ResStationType::AddSub, 3);
        cpu.add_exec_unit(ResStationType::MulDiv, 2);
        cpu.add_exec_unit(ResStationType::LoadBuffer, 2);
        cpu
    }

    pub fn set_regs(&mut self, index: usize, number: isize) {
        self.regs[index] = number;
    }

    pub fn done(&self) -> bool {
        self.done
    }

    fn debug_rob(&self) {
        for i in 0..self.rob.len() {
            println!("index: {}, rob: {:?}", i, self.rob[i]);
        }
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
                        inst: None,
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

    /// 添加执行单元
    fn add_exec_unit(&mut self, rs_type: ResStationType, count: usize) {
        for _ in 0..count {
            self.exec_units.push(ExecUint {
                busy: false,
                rs_type: rs_type,
                cycles: 0,
                rs_index: 0
            });
        }
    }

    fn find_reorder(&self, addr: usize) -> Option<usize> {
        self.rob.iter().position(|item| item as *const _ as usize == addr) 
    }

    /// 发现空闲的执行单元
    fn find_empty_exec_unit(&self, rs_type: ResStationType) -> Option<usize> {
        for i in 0..self.exec_units.len() {
            if !self.exec_units[i].busy && self.exec_units[i].rs_type == rs_type {
                return Some(i)
            }
        }
        None
    }

    // /// 发射操作数，即将操作数写入到保留站中
    fn issue_op(&mut self, reg_index: usize, rs: usize, op: usize) {
        // 发射操作数
        let reg_stat = &self.reg_stat;
        // 如果操作数 1 的目前的状态是 busy 表示当前操作数不在寄存器中
        // 而将要被前面的指令写回或者在 ROB 中
        if reg_stat[reg_index].busy  {
            // 获取 reorder_addr 地址的值
            let reorder_addr = reg_stat[reg_index].reorder.unwrap();
            println!("[Debug] reorder_addr: {:#x}", reorder_addr);
            let reorder_index = self.find_reorder(reorder_addr).unwrap();
            let rs = &mut self.rs[rs];
            if self.rob[reorder_index].ready {
                // 在 ROB 中已经将该寄存器的值计算完成，但仍然没有 commit
                // 此时直接进行赋值即可
                if op == 1 {
                    rs.inner.rs_value = Some(self.rob[reorder_index].inner.value.unwrap());
                    rs.inner.rs_index = None;
                }else if op == 2 {
                    rs.inner.rt_value= Some(self.rob[reorder_index].inner.value.unwrap());
                    rs.inner.rt_index = None;
                }
            }else{
                // 此时在 ROB 仍然没有计算完, 记录为 ROB 的序号
                if op == 1 {
                    rs.inner.rs_index = Some(reorder_addr);
                }else if op == 2 {
                    rs.inner.rt_index = Some(reorder_addr);
                }
            }
            rs.busy = true;
        }else{
            let rs = &mut self.rs[rs];
            // 目前操作数在寄存器堆中
            rs.busy = true;
            if op == 1 {
                rs.inner.rs_value = Some(self.regs[reg_index] as usize);
                rs.inner.rs_index = None;
            }else if op == 2 {
                rs.inner.rt_value = Some(self.regs[reg_index] as usize);
                rs.inner.rt_index = None;
            }
            println!("[Debug] reg{}: {}", reg_index, self.regs[reg_index]);
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
        if let Some(inst) = self.instruction_queue.pop_front() {
            let rs_type: ResStationType = inst.into();
            if let Some((rs, rob)) = self.can_issue(rs_type) {
                match rs_type {
                    // 浮点数运算操作
                    ResStationType::AddSub | ResStationType::MulDiv => {
                        match inst {
                            Instruction::Add(op) | Instruction::Sub(op) | Instruction::Mul(op) | Instruction::Div(op) => {
                                let r1 = op.operand1 as usize;
                                let r2 = op.operand2 as usize;
                                let rd = op.target as usize;

                                // 发射操作数
                                self.issue_op(r1, rs, 1);
                                self.issue_op(r2, rs, 2);
                                
                                let rs = &mut self.rs[rs];
                                rs.inner.inst = Some(inst);
                                // 设置目标寄存器状态
                                self.reg_stat[rd].reorder = Some(&self.rob[rob] as *const _ as usize);
                                self.reg_stat[rd].busy = true;
                                self.rob[rob].inner.dest = Some(rd);
                                self.rob[rob].busy = true;
                                self.rob[rob].ready = false;
                                self.rob[rob].inner.inst = Some(inst);
                            },

                            _ => {}
                        }
                    },
                    ResStationType::LoadBuffer => {
                        match inst {
                            // Instruction::Ld(op) => {

                            // },
                            // Instruction::Sd(op) => {

                            // },
                            _ => {}
                        }
                    }
                }
            }
        }   
    }

    /// 执行指令
    pub(crate) fn exec(&mut self) {
        // 遍历保留站检查有哪些写指令可以开始执行
        for rs_index in 0..self.rs.len() {
            if self.rs[rs_index].inner.rs_index.is_none() && self.rs[rs_index].inner.rt_index.is_none() && self.rs[rs_index].busy {
                let inst = self.rs[rs_index].inner.inst.unwrap();
                let rs_type: ResStationType = inst.into();
                if let Some(exec_unit_index) = self.find_empty_exec_unit(rs_type) {
                    self.exec_units[exec_unit_index].busy = true;
                    match inst {
                        Instruction::Add(_) => { self.exec_units[exec_unit_index].cycles = ADD_CYCLES },
                        Instruction::Sub(_) => { self.exec_units[exec_unit_index].cycles = SUB_CYCLES },
                        Instruction::Mul(_) => { self.exec_units[exec_unit_index].cycles = MUL_CYCLES },
                        Instruction::Div(_) => { self.exec_units[exec_unit_index].cycles = DIV_CYCLES },
                        _ => {}
                    }
                    // 执行单元获取保留站的索引
                    self.exec_units[exec_unit_index].rs_index = rs_index;
                }else{
                    println!("[Debug] No ExecUnit to execute instruction");
                }
            }
        }
    }

    /// 将结果写到 CDB 总线并进行广播
    pub(crate) fn write_result(&mut self) {
        for i in 0..self.exec_units.len() {
            if self.exec_units[i].busy && self.exec_units[i].cycles > 0{
                self.exec_units[i].cycles -= 1;
            }
            if self.exec_units[i].cycles == 0 && self.exec_units[i].busy {
                // 当执行所需周期为 0 时，需要计算结果并将其送到 CDB 总线上
                let rs_index = self.exec_units[i].rs_index;
                let res_station = &mut self.rs[rs_index];
                let inst = res_station.inner.inst.unwrap();
                let mut res: usize = 0;
                let mut rd: usize = 0;
                match inst {
                    Instruction::Add(op) => {
                        res = res_station.inner.rs_value.unwrap() + res_station.inner.rt_value.unwrap();
                        rd = op.target as usize;
                    },
                    _ => {

                    }
                }
                // 获取到 reorder 的地址
                let dest = self.reg_stat[rd].reorder.unwrap();
                // 获取 rob 的索引
                let rob_index = self.find_reorder(dest).unwrap();
                // 将寄存器状态设为空闲，表示没有指令将会写入到
                // 寄存器中
                self.reg_stat[rd].busy = false;
                // 将依赖于该寄存器的保留站的操作数写入
                // 模拟的是 CDB 的广播
                for rs_item in self.rs.iter_mut() {
                    if rs_item.inner.rs_index == Some(dest) {
                        rs_item.inner.rs_value = Some(res);
                        rs_item.inner.rs_index = None;
                    }
                    if rs_item.inner.rt_index == Some(dest) {
                        rs_item.inner.rt_value = Some(res);
                        rs_item.inner.rt_index = None;
                    }
                }
                // 将 ROB ready 设置为 true，表示可以进行提交了
                self.rob[rob_index].ready = true;
                self.rob[rob_index].inner.value = Some(res);
            }
        }
    }

    /// 提交指令
    pub(crate) fn commit(&mut self) {
        // 检查 ROB 头部的指令是否能被提交
        if self.rob[0].busy == false {
            self.done = true;
        }else{
            while self.rob[0].ready {
                let rob_head = &self.rob[0];
                let inst = rob_head.inner.inst.unwrap();
                let rs_type: ResStationType = inst.into();
                // 获取写回寄存器的编号
                let dest = rob_head.inner.dest.unwrap();
                match rs_type {
                    ResStationType::AddSub | ResStationType::MulDiv => {
                        // 浮点数操作直接将计算的值写回到寄存器堆中
                        self.regs[dest] = rob_head.inner.value.unwrap() as isize;
                        // println!("[Debug] commit: reg{}: {}", dest, rob_head.inner.value.unwrap() as isize);
                    },
                    _ => {
    
                    }
                }
                if self.reg_stat[dest].reorder == Some(rob_head as *const _ as usize) {
                    self.reg_stat[dest].busy = false;
                }
                // 设置寄存器状态，将被占用的寄存器状态设置为空
                let mut rob_head = self.rob.remove(0);
                rob_head.busy = false;
                self.rob.push(rob_head);
                
            }
        }
        
    }

    /// 在一周期内所执行的操作
    /// 包括发射、执行、写结果、提交
    pub(crate) fn single_cycle(&mut self) {
        // 将周期添加 1
        self.cycles += 1;
        // 进行指令发射
        self.issue();
        // 检查保留站开始执行指令
        self.exec();
        // 将结果写到 CDB 总线并进行广播
        self.write_result();
        // 进行指令提交
        self.commit();
        // println!("[Debug] cycles: {}", self.cycles);
    }

}