use tomasulo_emulator::{cpu::{ TomasuloCpu, Cpu, Operand, Instruction }, trace::Trace};

fn main() {
    // fp_test();
    load_test();
}

fn fp_test() {
    let mut trace = Trace::new("traces/tomasulo.txt");
    let mut cpu = TomasuloCpu::new(&mut trace);
    cpu.set_regs(2, 100);
    cpu.set_regs(3, 200);
    cpu.set_regs(1, 2);
    cpu.add_inst(Instruction::Add(Operand::new(1, 2, 3)));
    cpu.add_inst(Instruction::Sub(Operand::new(1, 3, 2)));
    cpu.add_inst(Instruction::Div(Operand::new(2, 3, 1)));
    cpu.add_inst(Instruction::Mul(Operand::new(1, 2, 3)));
    cpu.add_inst(Instruction::Add(Operand::new(3, 1, 2)));
    cpu.add_inst(Instruction::Mul(Operand::new(2, 1, 3)));
    cpu.add_inst(Instruction::Mul(Operand::new(2, 1, 3)));
    cpu.add_inst(Instruction::Div(Operand::new(2, 3, 1)));
    cpu.run();
}

fn load_test() {
    let mut trace = Trace::new("traces/tomasulo.txt");
    let mut cpu = TomasuloCpu::new(&mut trace);
    cpu.write_memory(0x0, 1);
    cpu.write_memory(0x4, 2);
    cpu.write_memory(0x8, 4);
    cpu.add_inst(Instruction::Ld(1, 0, 0x0));
    cpu.add_inst(Instruction::Ld(2, 0, 0x4));
    cpu.add_inst(Instruction::Ld(3, 0, 0x8));
    cpu.run();
}