use crate::mutex::Mutex;
use alloc::vec::Vec;
use alloc::vec;
use alloc::sync::Arc;

pub const RAM_SIZE: usize = 0x10000 * 4;

#[derive(Clone)]
pub struct SharedMemory {
    data: Vec<u8>,
    mutex: Arc<Mutex<()>>,
}

impl core::fmt::Debug for SharedMemory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.data.fmt(f)
    }
}

impl SharedMemory {
    pub fn new(size: usize) -> Self {
        SharedMemory {
            data: vec![0; size],
            mutex: Arc::new(Mutex::new(())),
        }
    }

    pub fn read(&self, offset: usize, buffer: &mut [u8]) {
        let _lock = self.mutex.lock();
        let offset = offset.checked_mul(4).expect("Invalid memory access");
        buffer.copy_from_slice(&self.data[offset..offset + buffer.len()]);
    }

    pub fn write(&mut self, offset: usize, data: &[u8]) {
        let lock = self.mutex.lock();
        let offset = if let Some(res) = offset.checked_mul(4) {
            res
        } else {
            panic!("Invalid memory access");
        };
        self.data[offset..offset + data.len()].copy_from_slice(data);

        drop(lock);
    }
}
