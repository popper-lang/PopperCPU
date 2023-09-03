use alloc::vec::Vec;
use crate::shared_mem::SharedMemory;
use crate::mmu::MMU;
use crate::parser::Binary;
use crate::stream::Stream;
use crate::shared_mem::RAM_SIZE;

#[derive(Debug)]
pub struct Cpu {
    pub pc: usize,
    pub sp: usize,
    pub registers: MMU,
    pub flag: MMU,
    pub ram: MMU,
    pub memory: SharedMemory,
    pub stack: Vec<u16>,
    pub debug_info: Stream
}

impl Cpu {
    pub fn new() -> Self {
        let memory = SharedMemory::new(RAM_SIZE);
        let register = memory.clone();
        let flag = memory.clone();
        let ram = memory.clone();

        Self {
            pc: 0,
            sp: 0,
            memory,
            registers: MMU::new(register, 0x1),
            flag: MMU::new(flag, 0x11),
            ram: MMU::new(ram, 0x111),
            stack: Vec::new(),
            debug_info: Stream::new(Vec::new())
        }
    }

    pub fn push_stack(&mut self, value: u16) {
        self.stack.push(value);
    }

    pub fn pop_stack(&mut self) -> u16 {
        self.stack.pop().unwrap()
    }

    pub fn read_stack(&self, index: usize) -> u16 {
        self.stack[index]
    }

    pub fn write_stack(&mut self, index: usize, value: u16) {
        self.stack[index] = value;
    }

    pub fn read_ram(&self, index: usize) -> [u8; 4] {
        let mut buffer = [0; 4];
        self.ram.read_memory(index, &mut buffer);
        buffer
    }

    pub fn write_ram(&mut self, index: usize, value: [u8; 4]) {
        self.ram.write_memory(index, &value);
    }

    pub fn read_register(&self, index: usize) -> [u8; 4] {
        let mut buffer = [0; 4];
        self.registers.read_memory(index, &mut buffer);
        buffer
    }

    pub fn write_register(&mut self, index: usize, value: [u8; 4]) {
        if index > 16 {
            panic!("Invalid register index");
        }

        self.registers.write_memory(index, &value);
    }

    pub fn read_flag(&self, index: usize) -> [u8; 4] {
        let mut buffer = [0; 4];
        self.flag.read_memory(index, &mut buffer);
        buffer
    }

    pub fn write_flag(&mut self, index: usize, value: &[u8]) {
        if index > 5 {
            panic!("Invalid flag index");
        }
        self.flag.write_memory(index, value);
    }

    pub fn write(&mut self, ty: u8, addr: u32, value: [u8; 4]) {
        match ty {
            0x01 => self.write_register(addr as usize, value),
            0x03 => self.write_ram(addr as usize, value),
            _ => panic!("Invalid operand type")
        }
    }

    pub fn read(&mut self, ty: u8, addr: u32) -> [u8; 4] {
        match ty {
            0x01 => self.read_register(addr as usize),
            0x03 => self.read_ram(addr as usize),
            _ => panic!("Invalid operand type")
        }
    }

    pub fn interpret(&mut self, command: Binary) {
        #[cfg(feature = "debug")]
        self.debug();

        match command.opcode {
            0x11 => { // mov
                let value = self.interpret_value((command.operand_type2, command.operand2));
                self.write(command.operand_type1, command.operand1, value);
            },
            0x12 => { // add
                let value = u32::from_le_bytes(self.interpret_value((command.operand_type2, command.operand2)));
                let result = u32::from_le_bytes(self.read(command.operand_type1, command.operand1)) + value ;
                self.write(command.operand_type1,command.operand1, result.to_le_bytes());
            },
            _ => panic!("Invalid opcode")
        }
    }

    pub fn interpret_value(&self, val: (u8, u32)) -> [u8; 4] {
        match val.0 {
            0x01 => self.read_register(val.1 as usize),
            0x02 => val.1.to_le_bytes(),
            0x03 => self.read_ram(val.1 as usize),
            _ => panic!("Invalid operand type")
        }
    }

    #[cfg(feature = "debug")]
    pub fn debug(&mut self) {
        self.write_ram(1, [0x1, 0x5, 0x4, 0x99]);
        self.write_ram(2, [0x2, 0x0, 0x0, 0x0]);
        self.write_ram(3, [0x3, 0x0, 0x0, 0x0]);
        self.write_ram(4, [0x4, 0x0, 0x0, 0x0]);
        println!("PC: {}", self.pc);
        println!("SP: {}", self.sp);
        let mut register_buffer: [u8; 4] = [0; 4];
        self.registers.read_memory(1, &mut register_buffer);
        println!("Registers: {:?}", register_buffer);
        let mut flag_buffer: [u8; 4] = [0; 4];
        self.flag.read_memory(1, &mut flag_buffer);
        println!("Flags: {:?}", flag_buffer);
        let mut ram_buffer: [u8; 4] = [0; 4];
        self.ram.read_memory(1, &mut ram_buffer);
        println!("RAM: {:?}", ram_buffer);
        println!("Stack: {:?}", self.stack);
    }
}



