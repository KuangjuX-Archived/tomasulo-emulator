use std::collections::HashMap;

/// 使用哈希表模拟内存
/// key: memory address value: memory value
pub(crate) struct Memory (HashMap<u32, i32>);

impl Memory {
    pub(crate) fn init() -> Self {
        Self(HashMap::new())
    }

    /// 从内存中读取数据
    pub(crate) fn read(&self, mut addr: u32) -> i32 {
        // 地址必须是 4 字节对齐的
        // assert_eq!(addr % 4, 0);
        if addr % 4 != 0 {
            addr = (addr / 4) * 4;
        }
        if let Some(val) = self.0.get(&addr) { *val }
        else{ 0 }
    }

    /// 向内存中写写数据
    pub(crate) fn write(&mut self, addr: u32, val: i32) {
        assert_eq!(addr % 4, 0);
        let _ = self.0.insert(addr, val);
    }
}