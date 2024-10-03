use std::io::Read;

use anyhow::anyhow;

use crate::error::NoTerminatingBitError;
use crate::BitVec;

pub struct BitReader<R> {
    term_bit: bool,
    inner: R,
}

impl<R: Read> BitReader<R> {
    /// Creates a new `BitReader` from a reader.
    pub fn new(reader: R, term_bit: bool) -> Self {
        BitReader {
            inner: reader,
            term_bit,
        }
    }

    /// Reads all the bits from the underlying reader.
    ///
    /// The encoded data should be written with the most-significant bit (MSB) first
    /// in big-endian byte order and should end with a terminating 1-bit.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, BitReader};
    /// use std::io::Cursor;
    ///
    /// let reader = Cursor::new(vec![0b10101011, 0b11001000]);
    /// let mut reader = BitReader::new(reader, true);
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

        if self.term_bit {
            with_terminating_bit(buffer)
        } else {
            Ok(BitVec::new(buffer))
        }
    }
}

// Returns the position of the trailing 1-bit.
// The position indexing starts from the right.
fn trailing_one_pos(byte: u8) -> Option<u8> {
    for i in 0..8 {
        if byte & (1 << i) != 0 {
            return Some(i);
        }
    }
    None // No 1-bit found
}

// Converts a buffer into a `BitVec`, removing the terminating bit.
//
// Searches for the terminating bit and returns an error if not found.
// If the terminating bit is at position 7, the last byte is removed;
// otherwise, the bit is cleared (set to 0). The resulting `BitVec`
// is then truncated to the correct length and returned.
fn with_terminating_bit(mut buffer: Vec<u8>) -> anyhow::Result<BitVec> {
    let &byte = buffer
        .last()
        .expect("The buffer is guaranteed to not be empty.");
    let term_bit_pos = trailing_one_pos(byte);
    return match term_bit_pos {
        None => Err(anyhow!(NoTerminatingBitError)),
        Some(pos) => {
            if pos == 7 {
                buffer.pop();
                let len = buffer.len() * 8;
                Ok(BitVec::with_len(buffer, len)?)
            } else {
                let byte = buffer
                    .last_mut()
                    .expect("The buffer is guaranteed to not be empty.");
                *byte &= !(1 << pos);
                let len = (buffer.len() - 1) * 8 + (7 - pos) as usize;
                Ok(BitVec::with_len(buffer, len)?)
            }
        }
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{bitvec, BitVec};
    use std::io::Cursor;

    #[test]
    fn test_empty_bitvec() {
        let reader = Cursor::new(Vec::<u8>::new());
        let reader = BitReader::new(reader, true);
        let bitvec = reader.read_to_end().unwrap();
        assert!(bitvec.is_empty());
    }

    #[test]
    fn test_bitvec_read() {
        let reader = Cursor::new(vec![0b10001100, 0b10000000]);
        let reader = BitReader::new(reader, true);
        let bitvec = reader.read_to_end().unwrap();
        assert_eq!(*bitvec.as_bytes(), [0b10001100]);
    }

    #[test]
    fn test_trailing_one_pos() {
        assert_eq!(trailing_one_pos(0), None);
        assert_eq!(trailing_one_pos(0b10010000), Some(4));
        assert_eq!(trailing_one_pos(0b10000000), Some(7));
    }

    #[test]
    fn test_with_terminating_bit() {
        let bv = bitvec![true, false, false];
        assert_eq!(with_terminating_bit(vec![0b10010000]).unwrap(), bv);

        let bv = bitvec![true, false, true, true, false, false, false, true];
        assert_eq!(
            with_terminating_bit(vec![0b10110001, 0b10000000]).unwrap(),
            bv
        );
    }
}
