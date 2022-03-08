
/// 1024 bytes * 4 = 4KB
pub(crate) struct Memory ([i32; 1024]);

impl Memory {
    pub(crate) fn init() -> Self {
        Self([0i32; 1024])
    }
}