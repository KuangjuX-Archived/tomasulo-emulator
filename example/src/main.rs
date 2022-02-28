use std::env;
use tomasulo_emulator::cpu::{ SingleCycleCpu, Cpu };

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("[Error] Please input file");
    }
    let input_file = args[1].clone();
    let mut cpu = SingleCycleCpu::new();
    cpu.read_inst(input_file).unwrap();
    cpu.execute();
}
