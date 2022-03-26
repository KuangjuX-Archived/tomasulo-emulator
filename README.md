# tomasulo-emulator
## Introuction
使用 Rust 语言实现的 Tomasulo + Hardware Speculation 的模拟器，支持少量指令，使用 `parser` 解析指令，和 CPU 的实现分离开。使用 `trace` 来追踪记录指令的运行状况，使用 `justfile` 来实现运行脚本。    
  
在本实验中，实现了 `ADD`, `SUB`, `MUL`, `DIV`, `LOAD` 等指令并且模拟了内存地址来进行运行。

- [x] FP operation
- [ ] Load/Store
- [ ] Branch

## Usage
```
just tomasulo
```
  
