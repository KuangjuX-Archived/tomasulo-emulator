use tomasulo_emulator::cpu::{ TomasuloCpu, Cpu, Operand, Instruction };

fn main() {
    let mut cpu = TomasuloCpu::new();
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