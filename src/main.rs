use std::env;

pub mod cpu;
pub mod parser;

use cpu::Cpu;
use cpu::common::SingleCycleCPU;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("[Error] Please input file");
    }
    let input_file = args[1].clone();
    let mut cpu = SingleCycleCPU::new();
    cpu.read_inst(input_file).unwrap();
    cpu.execute();
}
