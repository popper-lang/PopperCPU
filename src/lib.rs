#![cfg_attr(
    not(feature = "debug"),
    no_std
)]
#![feature(error_in_core)]

extern crate alloc;

use std::fs::read_to_string;
use crate::parser::Binary;

pub mod stream;
pub mod shared_mem;
pub mod cpu;
pub mod parser;
pub mod mmu;
mod mutex;
mod plugin;


#[cfg(feature = "parse")]
pub fn compile_to_binary_file(file_name: &str) -> Vec<Binary> {
    let body = read_to_string(file_name).unwrap();
    let binary = compile_to_bin_string(body.as_str());
    binary
}

#[cfg(feature = "parse")]
pub fn compile_to_bin_string(body: &str) -> Vec<Binary>  {
    use parser::bin_parser::BinParser;
    let mut parser = BinParser::new(body);
    let binary = parser.compiles().unwrap();
    binary
}

#[cfg(feature = "parse")]
pub fn interpret_string(string: &str) {
    interpret_binary(compile_to_bin_string(string))
}

#[cfg(feature = "parse")]
pub fn interpret_file(file_name: &str) {
    interpret_binary(compile_to_binary_file(file_name))
}

pub fn interpret_binary(binary: Vec<Binary>) {
    use cpu::Cpu;
    let mut cpu = Cpu::new(binary);
    cpu.interpret();
}


