use alloc::vec::Vec;
use core::error::Error;
use core::fmt::{Debug, Display};

trait Write<Err: Error> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Err>;
    fn flush(&mut self) -> Result<(), Err>;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Err>;
}

#[derive(Clone)]
pub struct Stream {
    pub bytes: Vec<u8>
}

impl Stream {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes}
    }

    pub fn read(&self, index: usize) -> u8 {
        self.bytes[index]
    }

    pub fn write(&mut self, index: usize, byte: u8) {
        self.bytes[index] = byte;
    }

    pub fn clear_all(&mut self) {
        for i in 0..self.bytes.len() {
            self.write(i, 0);
        }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub unsafe fn to_string(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.bytes)
        }
    }

}

impl Debug for Stream {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Stream[{}]", unsafe { self.to_string() })
    }
}

#[derive(Debug)]
pub struct StreamError<'a> {
    message: &'a str
}

impl<'a> Display for StreamError<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<'a> Error for StreamError<'a> {}


impl<'a> Write<StreamError<'a>> for Stream {
    fn write(&mut self, buf: &[u8]) -> Result<usize, StreamError<'a>> {
        let mut i = 0;
        for byte in buf {
            self.write(i, *byte);
            i += 1;
        }
        Ok(i)
    }

    fn flush(&mut self) -> Result<(), StreamError<'a>> {
        self.clear_all();
        Ok(())
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<(), StreamError<'a>> {
        for (i, byte) in buf.iter().enumerate() {
            self.write(i, *byte);
        }
        Ok(())
    }
}

pub struct IOStream {
    input: Stream,
    output: Stream,
    error: Stream
}

impl IOStream {
    pub fn new(input: Stream, output: Stream, error: Stream) -> Self {
        Self { input, output, error }
    }
}

pub const INPUT: u8 = 0;
pub const OUTPUT: u8 = 1;
pub const ERROR: u8 = 2;

pub fn write_io<'a>(id: u8, content: &'a [u8], iostream: &mut IOStream) -> Result<&'a [u8], StreamError<'a>> {
    match id {
        INPUT => iostream.input.write_all(content)?,
        OUTPUT => iostream.output.write_all(content)?,
        ERROR => iostream.error.write_all(content)?,
        _ => return Err(StreamError { message: "bad id" })
    };

    Ok(content)
}





