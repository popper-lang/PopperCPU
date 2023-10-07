use alloc::vec::Vec;
use crate::shared_mem::SharedMemory;
use crate::mmu::MMU;
use crate::parser::Binary;
use crate::stream::Stream;
use crate::shared_mem::RAM_SIZE;

macro_rules! bytes {
    ($expr:expr) => {
        u32::from_le_bytes($expr)
    };
}

macro_rules! read_into_buffer {
    ($name_mem:expr, $offset:expr) => { {
        let mut buffer = [0; 4];
        $name_mem.read_memory($offset, &mut buffer);
        buffer
    }};
}

#[derive(Debug)]
pub struct Cpu {
    pub pc: usize,
    pub sp: usize,
    pub registers: MMU,
    pub flag: MMU,
    pub ram: MMU,
    pub instructions: MMU,
    pub memory: SharedMemory,
    pub stack: Vec<u16>,
    pub debug_info: Stream,
    pub binary: Vec<Binary>
}

impl Cpu {
    pub fn new(binary: Vec<Binary>) -> Self {
        let memory = SharedMemory::new(RAM_SIZE);
        let register = memory.clone();
        let flag = memory.clone();
        let ram = memory.clone();
        let instructions = memory.clone();

        Self {
            pc: 0,
            sp: 0,
            memory,
            registers: MMU::new(register, 0x1),
            flag: MMU::new(flag, 0x11),
            ram: MMU::new(ram, 0x111),
            instructions: MMU::new(instructions, 0x1111),
            stack: Vec::new(),
            debug_info: Stream::new(Vec::new()),
            binary
        }
    }

    pub fn load_instructions(&mut self) {
        let mut labels = Vec::new();
        for bin in self.binary.iter() {
            let bytes = bin.to_bytes();
            let mut buffer = [0; 4];
            buffer.copy_from_slice(&bytes[0..4]);
            let mut index = (u32::from_le_bytes(buffer) as usize - 1)  * 5;
            if !labels.is_empty() {
                index = labels.pop().unwrap() + 5;
            }
            labels.push(index);
            self.instructions.write_memory(index, &(bytes[4] as u32).to_le_bytes());
            self.instructions.write_memory(index + 1, &(bytes[5] as u32).to_le_bytes());
            self.instructions.write_memory(index + 2, &bytes[6..11]);
            self.instructions.write_memory(index + 3, &(bytes[10] as u32).to_le_bytes());
            self.instructions.write_memory(index + 4, &bytes[11..]);

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

    pub fn write(&mut self, ty: u8, addr: [u8; 4], value: [u8; 4]) {
        match ty {
            0x01 => self.write_register(bytes!(addr) as usize, value),
            0x04 => self.write_ram(bytes!(addr) as usize, value),
            e => panic!("Invalid operand type:{:?}", e)
        }
    }

    pub fn read(&mut self, ty: u8, addr: [u8; 4]) -> [u8; 4] {
        match ty {
            0x01 => self.read_register(bytes!(addr) as usize),
            0x04 => self.read_ram(bytes!(addr) as usize),
            _ => panic!("Invalid operand type")
        }
    }

    pub fn next_bytes(&mut self) -> [u8; 4] {
        let mut buffer = [0; 4];

        self.instructions.read_memory(self.pc, &mut buffer);
        self.pc += 1;
        buffer
    }


    pub fn interpret(&mut self) {
        self.load_instructions();

        while read_into_buffer!(self.flag, 0) == [0; 4] {
            self.interpret_instruction();
        }

        println!("{}", bytes!(read_into_buffer!(self.ram, 500)))
    }

    pub fn interpret_instruction(&mut self) {
        let opcode_buffer = self.next_bytes();
        let opcode = bytes!(opcode_buffer) as u8;
        match opcode {
            0x0 => self.flag.write_memory(0, &[1, 0, 0, 0]),
            0x11 => { // mov
                let operand1_type = bytes!(self.next_bytes()) as u8;
                let operand1 = self.next_bytes();
                let operand2_type = bytes!(self.next_bytes()) as u8;
                let operand2 = self.next_bytes();
                let value = self.interpret_value((operand2_type, operand2));
                self.write(operand1_type, operand1, value);
            },
            0x12 => { // add
                let operand1_type = bytes!(self.next_bytes()) as u8;
                let operand1 = self.next_bytes();
                let operand2_type = bytes!(self.next_bytes()) as u8;
                let operand2 = self.next_bytes();
                let value = bytes!(self.interpret_value((operand2_type, operand2)));
                let result = bytes!(self.read(operand1_type, operand1)) + value ;
                self.write(operand1_type,operand1, result.to_le_bytes());
            },
            0x17 => { // call
                let operand1_type = bytes!(self.next_bytes()) as u8;
                let operand1 = self.next_bytes();
                self.write_ram(100, (self.pc as u32).to_le_bytes());
                if operand1_type == 0x3 {
                    self.pc = (bytes!(operand1) as usize - 1) *  5;
                } else {
                    panic!("Invalid operand type: {}", operand1_type)
                }
            },
            0x19 => { // int
                let (_, _, _, _) = (self.next_bytes(), self.next_bytes(), self.next_bytes(), self.next_bytes());

                // end of the program
                self.flag.write_memory(0, &[0x1, 0x0, 0x0, 0x0]);
            },
            0x21 => { // ret
                self.next_bytes();
                self.next_bytes();
                self.next_bytes();
                self.next_bytes();
                match bytes!(self.read_ram(100)) {
                    0 => self.flag.write_memory(0, &[1, 0, 0, 0]),
                    e => {
                        self.pc = (e * 5 + 2) as usize;
                        self.write_ram(100, [0; 4]);
                    }
                }
            }
            e => panic!("Invalid opcode: {:?}", e)
        };

    }

    pub fn interpret_value(&self, val: (u8, [u8; 4])) -> [u8; 4] {
        match val.0 {
            0x01 => self.read_register(u32::from_le_bytes(val.1) as usize),
            0x02 => val.1,
            0x03 => self.read_ram(u32::from_le_bytes(val.1) as usize),
            e => panic!("Invalid operand type {:?}", e)
        }
    }

    #[cfg(feature = "debug")]
    pub fn debug(&mut self) {
        self.ram.write_memory(1, &[0x5, 0x1, 0x0, 0x0]);
        println!("PC: {}", self.pc);
        println!("SP: {}", self.sp);
        let mut register_buffer: [u8; 4] = [0; 4];
        self.registers.read_memory(1, &mut register_buffer);
        println!("Register r1: {:?}", register_buffer);
        let mut flag_buffer: [u8; 4] = [0; 4];
        self.flag.read_memory(1, &mut flag_buffer);
        println!("Flag zc: {:?}", flag_buffer);
        let mut ram_buffer: [u8; 4] = [0; 4];
        self.ram.read_memory(500, &mut ram_buffer);
        println!("RAM {:#x}: {:?}",  500, ram_buffer);
        println!("Stack: {:?}", self.stack);
        let mut instruction_buffer = vec![[0; 4]; 4 * 5];
        let index = 10;
        self.instructions.read_memory(index * 5, &mut instruction_buffer[0]);
        self.instructions.read_memory(index * 5 + 1, &mut instruction_buffer[1]);
        self.instructions.read_memory(index * 5 + 2, &mut instruction_buffer[2]);
        self.instructions.read_memory(index * 5 + 3, &mut instruction_buffer[3]);
        self.instructions.read_memory(index * 5 + 4, &mut instruction_buffer[4]);

        println!("Instruction : {:?}", instruction_buffer[0]);
        println!("Instruction : {:?}", instruction_buffer[1]);
        println!("Instruction : {:?}", instruction_buffer[2]);
        println!("Instruction : {:?}", instruction_buffer[3]);
        println!("Instruction : {:?}", instruction_buffer[4]);


    }
}



