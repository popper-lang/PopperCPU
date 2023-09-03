
use crate::shared_mem::SharedMemory;

#[derive(Debug)]
pub struct MMU {
    shared_memory: SharedMemory,
    base_address: usize
}

impl MMU {
    pub fn new(shared_memory:  SharedMemory, base_address: usize) -> Self {
        MMU { shared_memory, base_address }
    }

    pub fn read_memory(&self, offset: usize, buffer: &mut [u8]) {
        self.shared_memory.read(offset + self.base_address, buffer);
    }

    pub fn write_memory(&mut self, offset: usize, data: &[u8]) {
        self.shared_memory.write(offset + self.base_address, data);
    }

}