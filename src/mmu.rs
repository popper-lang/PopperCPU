
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



// impl Index<usize> for MMU {
//     type Output = u32;
//
//     fn index(&self, index: usize) -> &Self::Output {
//         self.mem.memory.index(index + self.basic_address)
//     }
// }
//
// impl Index<Range<usize>> for MMU {
//     type Output = [u32];
//
//     fn index(&self, index: Range<usize>) -> &Self::Output {
//         &self.mem.memory[index.start + self.basic_address..index.end + self.basic_address]
//     }
// }
//
// impl Index<RangeFrom<usize>> for MMU {
//     type Output = [u32];
//
//     fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
//         &self.mem.memory[index.start + self.basic_address..]
//     }
// }
//
// impl Index<RangeFull> for MMU {
//     type Output = [u32];
//
//     fn index(&self, index: RangeFull) -> &Self::Output {
//         &self.mem.memory[index]
//     }
// }
//
// impl Index<RangeTo<usize>> for MMU {
//     type Output = [u32];
//
//     fn index(&self, index: RangeTo<usize>) -> &Self::Output {
//         &self.mem.memory[..index.end + self.basic_address]
//     }
// }
//
// impl Index<RangeToInclusive<usize>> for MMU {
//     type Output = [u32];
//
//     fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
//         &self.mem.memory[..=index.end + self.basic_address]
//     }
// }
//
// impl Index<RangeInclusive<usize>> for MMU {
//     type Output = [u32];
//
//     fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
//         &self.mem.memory[*index.start() + self.basic_address..=*index.end() + self.basic_address]
//     }
// }
//
//
