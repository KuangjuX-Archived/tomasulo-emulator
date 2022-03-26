use rand::Rng;
use tomasulo_emulator::cpu::{ SingleCycleCpu, Cpu };
use tomasulo_emulator::parser::Parser;
use tomasulo_emulator::trace::Trace;
use tomasulo_emulator::cpu::{ Instruction, Operand };



fn main() {
    let mut trace = Trace::new("traces/single_cycle.txt");
    let mut cpu = SingleCycleCpu::new(&mut trace);
    let mut rng = rand::thread_rng();
    for i in 0..32 {
        let rand_val = rng.gen_range(0..1000);
        cpu.set_regs(i, rand_val);
    }
    let parser = Parser::new();
    parser.read_inst(&mut cpu, "inst.txt").expect("Fail to read instruction");
    parser.read_data(&mut cpu, "data.txt").expect("Fail to read data");
    cpu.run();
}
