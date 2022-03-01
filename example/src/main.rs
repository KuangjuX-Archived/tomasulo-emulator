use std::env;
use tomasulo_emulator::cpu::{ SingleCycleCpu, Cpu };
use tomasulo_emulator::parser::Parser;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("[Error] Please input file");
    }
    let input_file = args[1].clone();
    let parser = Parser::new();
    let mut cpu = SingleCycleCpu::new();
    parser.read_inst(&mut cpu, input_file).unwrap();
    cpu.execute();
}
