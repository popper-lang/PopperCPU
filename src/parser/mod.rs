pub mod bin_parser;

#[derive(PartialEq, Debug)]
pub struct Binary {
    pub label: u32,
    pub opcode: u8,
    pub operand_type1: u8,
    pub operand1: u32,
    pub operand_type2: u8,
    pub operand2: u32,
}

