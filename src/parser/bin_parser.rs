use alloc::vec;
use super::Binary;
use alloc::vec::Vec;

pub static DEBUG: bool = false;

pub static LABEL_FMT_SIZE: usize = 8 * 4;
pub static OPCODE_FMT_SIZE: usize = 8;
pub static OPTYPE_FMT_SIZE: usize = 4;
pub static OPERAND_FMT_SIZE: usize = 8 * 4;

pub struct BinParser<'a> {
    pub bin: &'a str,
    pub index: usize,
}

impl<'a> BinParser<'a> {
    pub fn new(bin: &'a str) -> Self {
        Self {
            bin,
            index: 0,
        }
    }

    pub fn compile(&mut self) -> Result<Binary, &'a str> {
        let mut acc = LABEL_FMT_SIZE - 1;
        let label = self.compile_bin_le_bytes(&self.bin[0..acc]);
        let mut old_acc = acc;
        acc += OPCODE_FMT_SIZE;
        let opcode = u8::from_str_radix(&self.bin[old_acc..acc], 2).unwrap();
        old_acc = acc;
        acc += OPTYPE_FMT_SIZE;
        let operand1_type = u8::from_str_radix(&self.bin[old_acc..acc], 2).unwrap();
        old_acc = acc;
        acc += OPERAND_FMT_SIZE;
        let operand1 = self.compile_bin_le_bytes(&self.bin[old_acc..acc]);
        old_acc = acc;
        acc += OPTYPE_FMT_SIZE;
        let operand2_type = u8::from_str_radix(&self.bin[old_acc..acc], 2).unwrap();
        old_acc = acc;
        acc += OPERAND_FMT_SIZE;
        let operand2 = self.compile_bin_le_bytes(&self.bin[old_acc..acc]);

        Ok(Binary {
            label: label,
            opcode,
            operand_type1: operand1_type,
            operand1,
            operand_type2: operand2_type,
            operand2,
        })






        // 000000010000000000000000000000000010001000100000001000000000000000000000000001000000010000000000000000000000000



    }

    pub fn compile_bin_le_bytes(&self, string: &str) -> u32 {
        let mut byte_array: [u8; 4] = [0; 4];
        let bytes = string.chars()
            .collect::<Vec<char>>()
            .chunks(8)
            .map(|char_array| {
                char_array
                    .iter()
                    .skip_while(|x| x == &&'0')
                    .fold(0, |acc, &c| {
                        match c {
                            '0' => acc * 2,
                            '1' => acc * 2 + 1,
                            _ => {
                                panic!("Invalid binary string.");
                            }
                        }
                    })
            })
            .collect::<Vec<u8>>();

        byte_array.copy_from_slice(bytes.as_slice());

        u32::from_le_bytes(byte_array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_parser() {
        let content = "000000010000000000000000000000000010001000100000001000000000000000000000000001000000010000000000000000000000000";
        let mut parser = BinParser::new(content);
        let res = parser.compile();
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), Binary {
            label: 0x1,
            opcode: 0x11,
            operand_type1: 0x1,
            operand1: 0x1,
            operand_type2: 0x2,
            operand2: 0x2
        });
    }
}



