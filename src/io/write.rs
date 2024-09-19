use std::io::{self, BufWriter, Write};

use anyhow::Context;

use crate::collections::BitVec;

/// This structure represents a bit-writer.
pub struct BitWriter<W: ?Sized + Write> {
    // The buffer. Avoid using this like a normal `Vec` in common code paths.
    // That is, don't use `buf.push`, `buf.extend_from_slice`, or any other
    // methods that require bounds checking or the like. This makes an enormous
    // difference to performance (we may want to stop using a `Vec` entirely).
    buf: BitVec,
    // #30888: If the inner writer panics in a call to write, we don't want to
    // write the buffered data a second time in BufWriter's destructor. This
    // flag tells the Drop impl if it should skip the flush.
    // panicked: bool,
    inner: W,
}

impl<W: Write> BitWriter<W> {
    /// Creates a new bit-writer.
    pub fn new(writer: W) -> Self {
        BitWriter {
            buf: BitVec::new(),
            inner: writer,
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
    /// use idencode::io::BitWriter;
    ///
    /// let writer = Cursor::new(vec![]);
    /// let mut bw = BitWriter::new(writer);
    /// let bits = vec![true, true, false];
    /// for bit in bits {
    ///     bw.write_bit(bit).unwrap();
    /// }
    ///
    /// assert_eq!(*bw.get_ref(), [0b110]);
    /// ```
    pub fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        self.buf.push(bit);
        if *self.buf.bit_position() == 0 && !self.buf.is_empty() {
            let byte = *self
                .buf
                .as_slice()
                .get(0)
                .expect("It is guaranteed that at least one byte exists.");
            self.reset();
            self.inner.write(&[byte])?;
        }
        Ok(())
    }

    /// Pushes bits from a slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use idencode::io::BitWriter;
    ///
    /// let writer = Cursor::new(vec![]);
    /// let mut bw = BitWriter::new(writer);
    /// bw.write_bits(&[true, true, false, true, false, false, false, false]).unwrap();
    ///
    /// let result = bw.finalize().unwrap().into_inner();
    /// assert_eq!(result, [0b11010000]);
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
    pub fn get_ref(&self) -> &[u8] {
        self.buf.as_slice()
    }

    /// Acquires a mutable reference to the underlying writer.
    ///
    /// Note that the buffer does not contain the byte that is currently
    /// written. Also, note that this mutating the output/input state of
    /// the stream may corrupt this object, so care must be taken when
    /// using this method.
    pub fn get_mut(&mut self) -> &mut [u8] {
        self.buf.as_mut_slice()
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
    /// use idencode::io::BitWriter;
    ///
    /// let writer = Cursor::new(vec![]);
    /// let mut bw = BitWriter::new(writer);
    /// bw.write_bit(true).unwrap();
    /// bw.write_bit(false).unwrap();
    /// let result = bw.finalize().unwrap();
    /// // The final buffer will contain a single byte, with `10` (binary) padded from the
    /// // left to become `000000010`.
    /// assert_eq!(result.into_inner(), vec![0b00000010]);
    /// ```
    pub fn finalize(mut self) -> anyhow::Result<W> {
        self.inner.write_all(self.buf.as_slice()).context("")?;
        self.inner.flush().context("")?;
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
        let mut bw = BitWriter::new(writer);
        let bits = vec![
            false, false, false, false, false, false, true, true, false, false, false, false,
            false, false, false, true, false, true, false,
        ];
        bw.write_bits(&bits).unwrap();
        let result = bw.finalize().unwrap();
        assert_eq!(result.into_inner(), vec![3, 1, 2])
    }
}
