use std::{collections::VecDeque, io::Write};
use crate::trace::Trace;

use super::{ Instruction, Cpu, Memory };

use rand::prelude::*;

pub const ADD_CYCLES: usize = 2;
pub const SUB_CYCLES: usize = 2;
pub const MUL_CYCLES: usize = 12;
pub const DIV_CYCLES: usize = 24;
pub const LOAD_CYCLES: usize = 2;
pub const JUMP_CYCLES: usize = 1;


#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum ResStationType {
    AddSub,
    MulDiv,
    LoadStore,
    JUMP
}

impl From<Instruction> for ResStationType {
    fn from(item: Instruction) -> ResStationType {
        match item {
            Instruction::Add(_) | Instruction::Sub(_) => { ResStationType::AddSub },
            Instruction::Mul(_) | Instruction::Div(_) => { ResStationType::MulDiv },
            Instruction::Ld(_, _, _) | Instruction::Sd(_, _, _) => { ResStationType::LoadStore },
            Instruction::Jump(_, _) => { ResStationType::JUMP },
            _ => { panic!("[Error] Invalid instruction") }
        }
    }
}

/// 保留站
#[derive(Debug)]
pub struct ReservedStation {
    /// 保留站类型
    rs_type: ResStationType,
    /// 是否被占用
    busy: bool,
    /// 是否已经被执行
    exec: bool,
    /// 保留站内部信息
    inner: ResStationInner
    
}

#[derive(Debug)]
pub struct ResStationInner {
    /// 内存地址，仅 load, store 使用
    address: Option<u32>,
    /// 指令类型
    inst: Option<Instruction>,
    /// Qj
    rs_index: Option<usize>,
    /// Qk
    rt_index: Option<usize>,
    /// Vj,
    rs_value: Option<i32>,
    /// Vk
    rt_value: Option<i32>,
    /// 记录 ROB 的地址
    dest: Option<usize>
}

/// ROB
#[derive(Debug)]
pub struct ReorderBuffer {
    busy: bool,
    ready: bool,
    index: usize,
    inner: ROBInner
}

impl ReorderBuffer {
    pub(crate) fn init() -> Self {
        let mut rng = rand::thread_rng();
        let rob = Self{
            busy: false,
            ready: false,
            index: rng.gen::<usize>(),
            inner: ROBInner {
                inst: None,
                dest: None,
                value: None
            }
        };
        rob
    }
}


#[derive(Debug)]
pub struct ROBInner {   
    inst: Option<Instruction>,
    /// 存储将写到寄存器的编号
    dest: Option<usize>,
    /// 存储计算的结果
    value: Option<i32>
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

#[derive(Debug)]
pub struct ExecUint {
    busy: bool,
    rs_type: ResStationType,
    cycles: usize,
    /// 保留站的索引
    rs_index: usize
}

pub struct TomasuloCpu<'a> {
    /// 是否完成
    done: bool,
    /// 周期数
    cycles: usize,
    /// 寄存器状态
    reg_stat: Vec<RegisterStatus>,
    /// 寄存器文件
    regs: [i32;32],
    /// 指令队列
    instruction_queue: VecDeque<Instruction>,
    /// 保留站
    rs: Vec<ReservedStation>,
    /// ROB
    rob: Vec<ReorderBuffer>,
    /// 执行单元
    exec_units: Vec<ExecUint>,
    /// 内存
    memory: Memory,
    /// 追踪文件
    trace: &'a mut Trace
}


impl<'a> Cpu for TomasuloCpu<'a> {
    fn add_inst(&mut self, inst: Instruction) {
        self.instruction_queue.push_back(inst);
    }

    fn run(&mut self) {
        loop {
            if !self.done(){ self.mult_issue(8); }
            else { break; }
        }
        println!("[Debug] Cpu run finished, cycles: {}", self.cycles);
    }

    fn trace<S>(&mut self, s: S) 
        where S: Into<String>
    {
        let s: String = s.into();
        writeln!(self.trace.file, "{}", s).unwrap();
    }

    fn write_memory(&mut self, addr: u32, val: i32) {
        self.memory.write(addr, val);
    }
}

impl<'a> TomasuloCpu<'a> {
    pub fn new(trace: &'a mut Trace) -> Self {
        let mut cpu = Self {
            done: false,
            cycles: 0,
            reg_stat: vec![RegisterStatus{ busy: false, reorder: None }; 32],
            regs: [0i32;32],
            instruction_queue: VecDeque::new(),
            rs: vec![],
            rob: vec![],
            exec_units: vec![],
            memory: Memory::init(),
            trace: trace
        };
        // 为 CPU 添加保留站
        cpu.add_rs(ResStationType::AddSub, 3);
        cpu.add_rs(ResStationType::MulDiv, 2);
        cpu.add_rs(ResStationType::LoadStore, 3);
        cpu.add_rs(ResStationType::JUMP, 3);
        // 为 CPU 添加 ROB
        cpu.add_rob(6);
        // 为 CPU 添加执行单元
        cpu.add_exec_unit(ResStationType::AddSub, 3);
        cpu.add_exec_unit(ResStationType::MulDiv, 2);
        cpu.add_exec_unit(ResStationType::LoadStore, 3);
        cpu.add_exec_unit(ResStationType::JUMP, 3);
        cpu
    }

    pub fn set_regs(&mut self, index: usize, number: i32) {
        self.regs[index] = number;
    }

    pub fn done(&self) -> bool {
        self.done
    }


    /// 添加保留站
    fn add_rs(&mut self, rs_type: ResStationType, count: usize) {
        for _ in 0..count {
            self.rs.push(
                ReservedStation { 
                    rs_type: rs_type,
                    busy: false,
                    exec: false,
                    inner: ResStationInner{
                        address: None,
                        inst: None,
                        rs_index: None,
                        rs_value: None,
                        rt_index: None,
                        rt_value: None,
                        dest: None
                    }
                }
            )
        }
    }

    /// 添加 ROB
    fn add_rob(&mut self, count: usize) {
        for _ in 0..count {
            self.rob.push(ReorderBuffer::init())
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
        self.rob.iter().position(|item| item.index == addr) 
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

    /// 发射操作数，即将操作数写入到保留站中
    fn issue_op(&mut self, reg_index: usize, rs: usize, op: usize) {
        // 发射操作数
        let reg_stat = &self.reg_stat;
        // 如果操作数目前的状态是 busy 表示当前操作数不在寄存器中
        // 而将要被前面的指令写回或者在 ROB 中
        if reg_stat[reg_index].busy  {
            // 获取 reorder_addr 地址的值
            let reorder_addr = reg_stat[reg_index].reorder.unwrap();
            let reorder_index = self.find_reorder(reorder_addr).unwrap();
            let rs = &mut self.rs[rs];
            if self.rob[reorder_index].ready {
                // 在 ROB 中已经将该寄存器的值计算完成，但仍然没有 commit
                // 此时直接进行赋值即可
                if op == 1 {
                    rs.inner.rs_value = Some(self.rob[reorder_index].inner.value.unwrap());
                    rs.inner.rs_index = None;
                }else if op == 2 {
                    rs.inner.rt_value = Some(self.rob[reorder_index].inner.value.unwrap());
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
        }else{
            let rs = &mut self.rs[rs];
            // 目前操作数在寄存器堆中
            if op == 1 {
                rs.inner.rs_value = Some(self.regs[reg_index]);
                rs.inner.rs_index = None;
            }else if op == 2 {
                rs.inner.rt_value = Some(self.regs[reg_index]);
                rs.inner.rt_index = None;
            }
        }
    }

    /// 查看是否能发射
    /// FP 操作需要看保留站是否有空闲, Load/Store 需要看 Buffer 是否有空闲
    fn can_issue(&self, rs_type: ResStationType) -> Option<(usize, usize)> {
        for i in 0..self.rs.len() {
            if !self.rs[i].busy && self.rs[i].rs_type == rs_type {
                for j in 0..self.rob.len() {
                    if !self.rob[j].busy {
                        return Some((i, j))
                    }
                }
            }
        }
        None
    }

    pub fn load_can_exec(&self, rs_index: usize) -> bool {
        if self.rs[rs_index].inner.rs_index.is_none() && self.rs[rs_index].busy && !self.rs[rs_index].exec {
            // let rob_index = self.rs[rs_index].inner.dest.unwrap();
            // let rob_index = self.find_reorder(rob_index).unwrap();
            // let rob = &self.rob[rob_index];
            // for i in 0..rob_index-1 {
            //     if let Some(item_inst) = self.rob[i].inner.inst {
            //         if let Instruction::Sd(_, _, _) = item_inst {

            //         }
            //     }
            // }
            return true
         }
        false
        // true
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
                                // 设置 ROB 地址
                                rs.inner.dest = Some(self.rob[rob].index);
                                rs.busy = true;
                                // 设置目标寄存器状态
                                self.reg_stat[rd].reorder = Some(self.rob[rob].index);
                                self.reg_stat[rd].busy = true;
                                // 设置 ROB 的信息
                                self.rob[rob].inner.dest = Some(rd);
                                self.rob[rob].busy = true;
                                self.rob[rob].ready = false;
                                self.rob[rob].inner.inst = Some(inst);
                            },

                            _ => {}
                        }
                    },
                    ResStationType::LoadStore => {
                        match inst {
                            Instruction::Ld(reg1, reg2, imm) => {
                                // 加载指令需要将 reg2 寄存器的内容 + imm 的值作为地址
                                // 并从内存中取出来存储到 reg1 中
                                // 首先需要发射操作数 2, 当等到其 Qj = 0 的时候才可以拿出来执行
                                self.issue_op(reg2, rs, 1);
                                let rs = &mut self.rs[rs];
        
                                rs.inner.inst = Some(inst);
                                rs.inner.address = Some(imm);
                                rs.busy = true;
                                rs.inner.dest = Some(self.rob[rob].index);

                                self.reg_stat[reg1].reorder = Some(self.rob[rob].index);
                                self.reg_stat[reg1].busy = true;

                                self.rob[rob].inner.dest = Some(reg1);
                                self.rob[rob].inner.inst = Some(inst);
                                self.rob[rob].busy = true;
                                self.rob[rob].ready = false;
                            },

                            Instruction::Sd(reg1, reg2, imm) => {

                            },
                            _ => { 
                                println!("[Error] inst: {:?}", inst);
                                panic!("Error instruction") 
                            }
                        }
                    },

                    ResStationType::JUMP => {
                        if let Instruction::Jump(r1, r2) = inst {
                             // 发射操作数
                             self.issue_op(r1, rs, 1);
                             self.issue_op(r2, rs, 2);
                             
                             let rs = &mut self.rs[rs];
                             rs.inner.inst = Some(inst);
                             // 设置 ROB 地址
                             rs.inner.dest = Some(self.rob[rob].index);
                             rs.busy = true;
                             // 由于没有目标寄存器，因此不需要设置目标寄存器状态
                             // 设置 ROB 的信息
                             self.rob[rob].busy = true;
                             self.rob[rob].ready = false;
                             self.rob[rob].inner.inst = Some(inst);
                        } 
                    }
                }
            }else {
                // 当目前没有足够的保留站时需要将其 push 到队列的顶部
                self.instruction_queue.push_front(inst);
            }
        }   
    }

    /// 执行指令
    pub(crate) fn exec(&mut self) {
        // 遍历保留站检查有哪些写指令可以开始执行
        for rs_index in 0..self.rs.len() {
            match self.rs[rs_index].rs_type {
                ResStationType::AddSub | ResStationType::MulDiv => {
                    if self.rs[rs_index].inner.rs_index.is_none() && self.rs[rs_index].inner.rt_index.is_none() && self.rs[rs_index].busy && !self.rs[rs_index].exec {
                        self.rs[rs_index].exec = true;
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
                        }
                    }
                },

                ResStationType::LoadStore => {
                    if let Some(inst) = self.rs[rs_index].inner.inst {
                        match inst {
                            Instruction::Ld(_, _, _) => {
                                 if self.load_can_exec(rs_index) {
                                     // 如果可以执行加载指令
                                     self.rs[rs_index].exec = true;
                                     let rs_type = self.rs[rs_index].rs_type;
                                     if let Some(exec_unit_index) = self.find_empty_exec_unit(rs_type) {
                                        // 执行单元获取保留站的索引
                                        self.exec_units[exec_unit_index].cycles = LOAD_CYCLES;
                                        self.exec_units[exec_unit_index].rs_index = rs_index;
                                        self.exec_units[exec_unit_index].busy = true;
                                     }
                                    
                                 }
                            },
                            Instruction::Sd(_, _, _) => {

                            },
                            _ => {panic!("get error instruction")}
                        }
                    }
                }

                ResStationType::JUMP => {
                    if self.rs[rs_index].inner.rs_index.is_none() && self.rs[rs_index].inner.rt_index.is_none() && self.rs[rs_index].busy && !self.rs[rs_index].exec {
                        self.rs[rs_index].exec = true;
                        let inst = self.rs[rs_index].inner.inst.unwrap();
                        let rs_type: ResStationType = inst.into();
                        if let Some(exec_unit_index) = self.find_empty_exec_unit(rs_type) {
                            self.exec_units[exec_unit_index].busy = true;
                            // 执行阶段什么都不做，独占一个周期
                            self.exec_units[exec_unit_index].cycles = JUMP_CYCLES;
                            // 执行单元获取保留站的索引
                            self.exec_units[exec_unit_index].rs_index = rs_index;
                        }
                    }
                }
            }
        }
    }

    /// 将结果写到 CDB 总线并进行广播
    pub(crate) fn write_result(&mut self) {
        for i in 0..self.exec_units.len() {
            if self.exec_units[i].busy && self.exec_units[i].cycles > 0 {
                self.exec_units[i].cycles -= 1;
            }
            if self.exec_units[i].cycles == 0 && self.exec_units[i].busy {
                // 当执行所需周期为 0 时，需要计算结果并将其送到 CDB 总线上
                let rs_index = self.exec_units[i].rs_index;
                let res_station = &mut self.rs[rs_index];
                let inst = res_station.inner.inst.unwrap();
                let mut res: i32 = 0;
                match inst {
                    Instruction::Add(_) => { res = if let Some(add_res) = res_station.inner.rs_value.unwrap().checked_add(res_station.inner.rt_value.unwrap()){ add_res }else{ 0 } },
                    Instruction::Sub(_) => { res = if let Some(sub_res) = res_station.inner.rs_value.unwrap().checked_sub(res_station.inner.rt_value.unwrap()){ sub_res }else{ 0 } },
                    Instruction::Mul(_) => { res = if let Some(mul_res) = res_station.inner.rs_value.unwrap().checked_mul(res_station.inner.rt_value.unwrap()){ mul_res }else{ 0 } },
                    Instruction::Div(_) => { res = if let Some(div_res) = res_station.inner.rs_value.unwrap().checked_div(res_station.inner.rt_value.unwrap()){ div_res }else { 0 } }
                    Instruction::Ld(_, _, _) => {
                        // load 指令的两步直接在一步做了
                        let addr = (res_station.inner.address.unwrap() as i32 + res_station.inner.rs_value.unwrap()) as u32;
                        res_station.inner.address = Some(addr);
                        res = self.memory.read(addr);
                    },
                    Instruction::Jump(_, _) => {
                        // 什么都不做
                    }
                    _ => { panic!("[Error] invalid instruction"); }
                }
                
                res_station.busy = false;
                res_station.exec = false;
                // 获取到 reorder 的地址
                let dest = res_station.inner.dest.unwrap();
                // 获取到 reorder 的地址
                let rob_index = self.find_reorder(dest).expect(format!("Invalid dest: {}", dest).as_str());
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
                // 将执行单元设置为空闲
                self.exec_units[i].busy = false;
            }
        }
    }

    /// 提交指令
    pub(crate) fn commit(&mut self) {
        // 检查 ROB 头部的指令是否能被提交
        if self.instruction_queue.len() == 0 && !self.rob[0].busy {
            self.done = true;
        }else{
            while self.rob[0].ready && self.rob[0].busy {
                let rob_head = &self.rob[0];
                let inst = rob_head.inner.inst.unwrap();
                let rs_type: ResStationType = inst.into();
                // 获取写回寄存器的编号
                if let Some(dest) = rob_head.inner.dest {
                    match rs_type {
                        ResStationType::AddSub | ResStationType::MulDiv | ResStationType::LoadStore => {
                            // 浮点数操作直接将计算的值写回到寄存器堆中
                            self.regs[dest] = rob_head.inner.value.unwrap();
                        },
                        _ => {}
                    }
                    // 将寄存器状态由 busy 修改为 free
                    if self.reg_stat[dest].reorder == Some(rob_head.index) && self.reg_stat[dest].busy {
                        self.reg_stat[dest].busy = false;
                        self.reg_stat[dest].reorder = None;
                    }
                }
                // 将 ROB 从 reorder 队列中 pop 出来
                self.rob.remove(0);
                // 重新 push 一个初始化的 ROB
                self.rob.push(ReorderBuffer::init());
                let mut info: String = String::new();
                for (index, reg) in self.regs.iter().enumerate() {
                    info.push_str(format!("reg{}: {}; ", index, reg).as_str());
                }
                self.trace(info);
                
            }
        }
        
    }

    /// 在一周期内所执行的操作
    /// 包括发射、执行、写结果、提交
    // pub(crate) fn single_cycle(&mut self) {
    //     // 将结果写到 CDB 总线并进行广播
    //     self.write_result();
    //     // 进行指令提交
    //     self.commit();
    //     // 将周期添加 1
    //     self.cycles += 1;
    //     // 进行指令发射
    //     self.issue();
    //     // 检查保留站开始执行指令
    //     self.exec();
    // }

    pub(crate) fn mult_issue(&mut self, issue_nums: usize) {
         // 将结果写到 CDB 总线并进行广播
         self.write_result();
         // 进行指令提交
         self.commit();
        // 将周期添加 1
        self.cycles += 1;
        // 进行多次指令发射
        for _ in 0..issue_nums {
            self.issue();
        }
        // 检查保留站开始执行指令
        self.exec();
    }

}