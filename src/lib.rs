pub mod cpu;
pub mod parser;
pub mod trace;



// #[cfg(test)]
// mod test {
//     use crate::cpu::{TomasuloCpu, Cpu, Instruction, Operand};


//     #[test]
//     fn run_test() {
//         let mut cpu = TomasuloCpu::new();
//         cpu.add_inst(Instruction::Add(Operand::new(1, 2, 3)));
//         cpu.add_inst(Instruction::Sub(Operand::new(1, 2, 3)));
//         cpu.run();
//     }
// }

