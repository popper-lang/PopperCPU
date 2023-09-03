#![cfg_attr(
    not(feature = "debug"),
    no_std
)]
#![feature(error_in_core)]


extern crate alloc;

pub mod stream;
pub mod shared_mem;
pub mod cpu;
pub mod parser;
pub mod mmu;
mod mutex;
