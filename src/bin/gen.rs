use std::fs::File;
use std::io::Write;
use rand::Rng;
fn main() {
    // 随机生成指令和内存文件
    let mut rng = rand::thread_rng();
    let mut inst_file = File::create("inst.txt").unwrap();
    let mut data_file = File::create("data.txt").unwrap();

    let inst_type = vec!["ADD", "SUB", "MUL", "DIV", "LD"];
    for _ in 0..100 {
        let idx = rng.gen_range(0..5);
        match inst_type[idx] {
            "ADD" | "SUB" | "MUL" | "DIV" => {
                let target: usize = rng.gen_range(0..32);
                let r1: usize = rng.gen_range(0..32);
                let r2: usize = rng.gen_range(0..32);
                writeln!(inst_file, "{},R{},R{},R{}", inst_type[idx], target, r1, r2).unwrap();
            },
            "LD" => {
                let target: usize = rng.gen_range(0..32);
                let r1: usize = rng.gen_range(0..32);
                let imm: usize = rng.gen_range(0..10);
                writeln!(inst_file, "{},R{},R{},{}", inst_type[idx], target, r1, imm).unwrap()
            },
            _ => {}
        }
    }

    for i in 0..10000 {
        let addr = 4 * i;
        let val: usize = rng.gen_range(1..10);
        writeln!(data_file, "{}: {}", addr, val).unwrap();
    }
}