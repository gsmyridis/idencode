use std::io::{self, Write};

use crate::collections::BitVec;
use crate::io::DEFAULT_BUF_SIZE;

/// This structure represents a bit-writer.
pub struct BitWriter<W: ?Sized + Write> {
    buf: BitVec,
    term_bit: bool,
    inner: W,
}

impl<W: Write> BitWriter<W> {
    /// Creates a new `BufWriter<W>` with a default buffer capacity.
    pub fn new(inner: W, term_bit: bool) -> BitWriter<W> {
        BitWriter::with_capacity(DEFAULT_BUF_SIZE, inner, term_bit)
    }

    /// Creates a new `BitWriter<W>` with at least the specified buffer capacity.
    pub fn with_capacity(capacity: usize, inner: W, term_bit: bool) -> BitWriter<W> {
        BitWriter {
            inner,
            buf: BitVec::with_capacity(capacity),
            term_bit
        }
    }

    /// Writes the bits of a given value in a most-significant-bit-first (MSB-first)
    /// order.
    ///
    /// The first bit written is the most significant bit of the value, followed by
    /// the next most significant bit, down to the least significant bit.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use idencode::BitWriter;
    ///
    /// let writer = Cursor::new(vec![]);
    /// let mut bw = BitWriter::new(writer, true);
    /// let bits = vec![true, true, false];
    /// for bit in bits {
    ///     bw.write_bit(bit).unwrap();
    /// }
    ///
    /// assert_eq!(*bw.get_ref().as_bytes(), [0b11000000]);
    /// ```
    pub fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        self.buf.push(bit);
        Ok(())
    }

    /// Pushes bits from a slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use idencode::BitWriter;
    ///
    /// let writer = Cursor::new(vec![]);
    /// let mut bw = BitWriter::new(writer, true);
    /// bw.write_bits(&[true, true, false, true, false, false, false, false]).unwrap();
    ///
    /// let result = bw.finalize().unwrap().into_inner();
    /// assert_eq!(result, [0b11010000, 0b10000000]);
    /// ```
    pub fn write_bits(&mut self, bits: &[bool]) -> io::Result<()> {
        for bit in bits {
            self.write_bit(*bit)?;
        }
        Ok(())
    }

    /// Acquires a shared reference to the underlying buffer.
    ///
    /// Note that the buffer does not contain the byte that is currently
    /// written.
    pub fn get_ref(&self) -> &BitVec {
        &self.buf
    }

    /// Acquires a mutable reference to the underlying writer.
    ///
    /// Note that the buffer does not contain the byte that is currently
    /// written. Also, note that this mutating the output/input state of
    /// the stream may corrupt this object, so care must be taken when
    /// using this method.
    pub fn get_mut(&mut self) -> &mut BitVec {
        &mut self.buf
    }

    /// Resets the state of this bit-writer entirely, cleaning the underlying
    /// buffer, and resets the current byte and current bit's position.
    pub fn reset(&mut self) {
        self.buf.clear()
    }

    /// Consumes the bit-writer and finalizes the writing, returning the
    /// underlying buffer.
    ///
    /// If the current byte being written is not yet full (i.e., fewer than 8 bits have
    /// been written), the remaining bits are padded from the left with zeros before the
    /// final byte is pushed into the buffer. This ensures that the buffer always contains
    /// full bytes.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the final sequence of bytes written by the `BitWriter`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::io::Cursor;
    /// use idencode::BitWriter;
    ///
    /// let writer = Cursor::new(vec![]);
    /// let mut bw = BitWriter::new(writer, true);
    /// bw.write_bit(true).unwrap();
    /// bw.write_bit(false).unwrap();
    /// let result = bw.finalize().unwrap();
    /// assert_eq!(result.into_inner(), vec![0b10100000]);
    /// ```
    pub fn finalize(mut self) -> io::Result<W> {
        if self.buf.is_empty() {
            return Ok(self.inner);
        }
        if self.term_bit {
            self.buf.push(true); // Add the terminating bit.
        }
        self.inner.write_all(self.buf.as_bytes())?;
        self.inner.flush()?;
        Ok(self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_push_bits() {
        let writer = Cursor::new(vec![]);
        let mut bw = BitWriter::new(writer, true);
        let bits = vec![
            false, false, false, false, false, false, true, true, false, false, false, false,
            false, false, false, true,
        ];
        bw.write_bits(&bits).unwrap();
        let result = bw.finalize().unwrap();
        assert_eq!(
            result.into_inner(),
            vec![0b00000011, 0b00000001, 0b10000000]
        )
    }
}
