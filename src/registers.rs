
#[macro_export]
macro_rules! table_register {
    ($reg:ident = $value:expr) => {
        $crate::registers::TableRegister::new($crate::registers::Register::$reg, $value)
    };

    ($reg:ident) => {
        $crate::registers::TableRegister::new($crate::registers::Register::$reg, 0)
    };

    ($( $reg:ident = $value:expr ),*) => {
        [$( $crate::table_register!($reg = $value) ),*]
    };

    ($( $reg:ident ),*) => {
        [$( $crate::table_register!($reg) ),*]
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    R1 = 0b00001,
    R2 = 0b00010,
    R3 = 0b00011,
    R4 = 0b00100,
    R5 = 0b00101,
    R6 = 0b00110,
    R7 = 0b00111,
    R8 = 0b01000,
    R9 = 0b01001,
    R10 = 0b01010,
    R11 = 0b01011,
    R12 = 0b01100,
    R13 = 0b01101,
    R14 = 0b01110,
    R15 = 0b01111,
    Rbp = 0b10000,
}

const SIZE_REGISTER: usize = 16;

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0b00001 => Register::R1,
            0b00010 => Register::R2,
            0b00011 => Register::R3,
            0b00100 => Register::R4,
            0b00101 => Register::R5,
            0b00110 => Register::R6,
            0b00111 => Register::R7,
            0b01000 => Register::R8,
            0b01001 => Register::R9,
            0b01010 => Register::R10,
            0b01011 => Register::R11,
            0b01100 => Register::R12,
            0b01101 => Register::R13,
            0b01110 => Register::R14,
            0b01111 => Register::R15,
            0b10000 => Register::Rbp,
            _ => panic!("invalid register"),
        }
    }
}

impl From<[u8; 4]> for Register {
    fn from(value: [u8; 4]) -> Self {
        Register::from(u32::from_le_bytes(value) as u8)
    }
}


