

pub struct Flag {
    pub zero: u8,
    pub subtract: u8,
    pub half_carry: u8,
    pub carry: u8,
}

impl Flag {
    pub fn new() -> Self {
        Self {
            zero: 0,
            subtract: 0,
            half_carry: 0,
            carry: 0,
        }
    }

    pub fn clear_all(&mut self) {
        self.zero = 0;
        self.subtract = 0;
        self.half_carry = 0;
        self.carry = 0;
    }

    pub fn read(&self, index: usize) -> u8 {
        match index {
            0 => self.zero,
            1 => self.subtract,
            2 => self.half_carry,
            3 => self.carry,
            _ => panic!("Invalid flag index"),
        }
    }

    pub fn write(&mut self, index: usize, value: u8) {
        match index {
            0 => self.zero = value,
            1 => self.subtract = value,
            2 => self.half_carry = value,
            3 => self.carry = value,
            _ => panic!("Invalid flag index"),
        }
    }
}

