pub mod global;

use std::io::{self, Read, Write};

use crate::error::InvalidCodeError;
use crate::num::Numeric;

pub trait Encoder<W: Write> {
    /// Creates a new Encoder wrapping a writer.
    fn new(writer: W) -> Self;

    /// Encodes and writes the specified numbers in the wrapped writer.
    fn write<T: Numeric>(&mut self, nums: &[T]) -> io::Result<()>;

    /// Finalizes the encoding returning the wrapped writer.
    fn finalize(self) -> io::Result<W>;
}

pub trait Decoder<R: Read> {
    /// Creates a new Decoder wrapping a reader.
    fn new(reader: R) -> Self;

    /// Reads and decodes the encoded numbers in the wrapped reader.
    fn decode<T: Numeric>(self) -> Result<Vec<T>, InvalidCodeError>;
}
