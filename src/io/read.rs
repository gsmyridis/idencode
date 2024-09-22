use std::io::Read;

use anyhow::anyhow;

use crate::error::NoTerminatingBitError;
use crate::BitVec;

pub struct BitReader<R: Read> {
    inner: R
}

impl<R: Read> BitReader<R> {
    /// Creates a new `BitReader` from a reader.
    pub fn new(reader: R) -> Self {
        BitReader{ inner: reader }
    }

    /// Reads all the bits from
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, BitReader};
    /// use std::io::Cursor;
    ///
    /// let reader = Cursor::new(vec![0b10101011, 0b11001000]);
    /// let mut reader = BitReader::new(reader);
    /// let bitvec = reader.read_to_end().unwrap();
    /// assert_eq!(bitvec.len(), 12);
    /// assert_eq!(*bitvec.bit_position(), 4);
    /// assert_eq!(*bitvec.as_bytes(), [0b10101011, 0b11000000]);
    /// ```
    pub fn read_to_end(mut self) -> anyhow::Result<BitVec> {
        // Read all the bytes in the reader
        let mut buffer = vec![];
        self.inner.read_to_end(&mut buffer)?;
        // If it's empty, return an empty BitVec.
        if buffer.is_empty() {
            return Ok(BitVec::default());
        }

        // Find the terminating bit, and return a BitVec with the appropriate
        // buffer, and length.
        let &byte = buffer.last().expect("The buffer is guaranteed to not be empty.");
        let term_bit_pos = trailing_one_pos(byte);
        return match term_bit_pos {
            None => Err(anyhow!(NoTerminatingBitError)),
            Some(pos) => {
                // Set the terminating bit to 0. Calculate the bitvec length.
                // If the position of the terminating bit is 7, then we discard
                // the last byte.
                if pos == 7 {
                    buffer.pop();
                }
                let len = (buffer.len() - 1) * 8 + (7 - pos) as usize;
                dbg!("{:?}", &buffer);
                let byte = buffer.last_mut().expect("The buffer is guaranteed to not be empty.");
                dbg!(&byte);
                *byte &= !(1 << pos);
                dbg!(&byte);
                Ok(BitVec::new(buffer, len)?)
            }
        }
    }
}


/// Returns the position of the trailing 1-bit.
/// The position indexing starts from the right.
fn trailing_one_pos(byte: u8) -> Option<u8> {
    for i in 0..8 {
        if byte & (1 << i) != 0 {
            return Some(i);
        }
    }
    None // No 1-bit found
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_trailing_one_pos() {
        assert_eq!(trailing_one_pos(0), None);
        assert_eq!(trailing_one_pos(0b10010000), Some(4));
        assert_eq!(trailing_one_pos(0b10000000), Some(7));
    }
}

