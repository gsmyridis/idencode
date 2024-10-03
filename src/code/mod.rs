pub mod global;

use std::io::{self, Read, Write};

use crate::error::InvalidCodeError;
use crate::num::Numeric;

pub trait EncodeOne {
    /// Encodes a single number, returning a buffer of bits.
    fn encode_one<T: Numeric>(num: T) -> Vec<bool>;
}

pub trait DecodeOne {
    /// Decodes a buffer of bits to a single number.
    fn decode_one<T: Numeric>(bits: &[bool]) -> Result<T, InvalidCodeError>;
}

pub trait Encoder<W: Write> {
    /// Encodes and writes the specified numbers in the wrapped writer.
    fn encode<T: Numeric>(&mut self, nums: &[T]) -> io::Result<()>;

    /// Finalizes the encoding returning the wrapped writer.
    fn finalize(self) -> io::Result<W>;
}

pub trait Decoder<R: Read> {
    /// Reads and decodes the encoded numbers in the wrapped reader.
    fn decode<T: Numeric>(self) -> Result<Vec<T>, InvalidCodeError>;
}
