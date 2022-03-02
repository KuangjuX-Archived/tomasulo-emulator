use tomasulo_emulator::cpu::{ TomasuloCpu, Cpu, Operand, Instruction };

fn main() {
    let mut cpu = TomasuloCpu::new();
    cpu.add_inst(Instruction::Add(Operand::new(1, 2, 3)));
    cpu.add_inst(Instruction::Sub(Operand::new(1, 2, 3)));
    cpu.run();
}