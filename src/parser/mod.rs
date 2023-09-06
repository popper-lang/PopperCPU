use alloc::vec::Vec;

pub mod bin_parser;


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Binary {
    pub label: [u8; 4],
    pub opcode: u8,
    pub operand_type1: u8,
    pub operand1: [u8; 4],
    pub operand_type2: u8,
    pub operand2: [u8; 4],
}


impl Binary {
    pub fn to_bytes(&self) -> [u8; 15] {
        let mut buffer = [0; 15];
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.label);
        bytes.push(self.opcode);
        bytes.push(self.operand_type1);
        bytes.extend_from_slice(&self.operand1);
        bytes.push(self.operand_type2);
        bytes.extend_from_slice(&self.operand2);
        buffer.copy_from_slice(bytes.as_slice());
        #[cfg(feature = "debug")]
        println!("Binary: {:?}", buffer);
        buffer
    }
}
