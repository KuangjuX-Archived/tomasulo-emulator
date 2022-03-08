use std::collections::HashMap;

/// 使用哈希表模拟内存
/// key: memory address value: memory value
pub(crate) struct Memory (HashMap<u32, i32>);

impl Memory {
    pub(crate) fn init() -> Self {
        Self(HashMap::new())
    }
}