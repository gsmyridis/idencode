/// This structure represents a bit-writer.
#[derive(Default)]
pub struct BitWriter {
    buffer: Vec<u8>,
    current_byte: u8,
    bit_position: u8,
}

impl BitWriter {
    /// Creates a new bit-writer with underlying buffer with specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        BitWriter {
            buffer: Vec::with_capacity(capacity),
            current_byte: 0,
            bit_position: 0,
        }
    }

    /// Writes the bits of a given value in a most-significant-bit-first (MSB-first) order.
    ///
    /// The first bit written is the most significant bit of the value, followed by the next
    /// most significant bit, down to the least significant bit.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::write::BitWriter;
    ///
    /// let mut bw = BitWriter::default();
    /// bw.write_bit(true);
    /// bw.write_bit(true);
    /// bw.write_bit(false);
    /// bw.write_bit(true);
    /// bw.write_bit(false);
    /// bw.write_bit(false);
    /// bw.write_bit(false);
    /// bw.write_bit(false);
    ///
    /// assert_eq!(*bw.get_ref(), [0b11010000]);
    /// ```
    pub fn write_bit(&mut self, bit: bool) {
        let bit = bit as u8;
        self.current_byte |= bit << (7 - self.bit_position);
        self.bit_position += 1;
        if self.bit_position == 8 {
            self.buffer.push(self.current_byte);
            self.current_byte = 0;
            self.bit_position = 0;
        }
    }

    /// Pushes bits from an iterator.
    pub fn write_bits(&mut self, bits: &[bool]) {
        for bit in bits {
            self.write_bit(*bit);
        }
    }

    /// Acquires a shared reference to the underlying buffer.
    ///
    /// Note that the buffer does not contain the byte that is currently
    /// written.
    pub fn get_ref(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    /// Acquires a mutable reference to the underlying writer.
    ///
    /// Note that the buffer does not contain the byte that is currently
    /// written. Also, note that this mutating the output/input state of
    /// the stream may corrupt this object, so care must be taken when
    /// using this method.
    pub fn get_mut(&mut self) -> &mut [u8] {
        self.buffer.as_mut_slice()
    }

    /// Resets the state of this bit-writer entirely, cleaning the underlying
    /// buffer, and resets the current byte and current bit's position.
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.current_byte = 0;
        self.bit_position = 0;
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
    /// use idencode::write::BitWriter;
    ///
    /// let mut bw = BitWriter::default();
    /// bw.write_bit(true);
    /// bw.write_bit(false);
    /// let result = bw.finalize();
    /// // The final buffer will contain a single byte, with `10` (binary) padded from the
    /// // left to become `000000010`.
    /// assert_eq!(result, vec![0b00000010]);
    /// ```
    pub fn finalize(mut self) -> Vec<u8> {
        if self.bit_position > 0 {
            self.current_byte >>= 8 - self.bit_position;
            self.buffer.push(self.current_byte);
        }
        self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_bits() {
        let mut bw = BitWriter::default();
        let bits = vec![
            false, false, false, false, false, false, true, true,
            false, false, false, false, false, false, false, true,
            false, true, false,
        ];
        bw.write_bits(&bits);
        assert_eq!(bw.finalize(), vec![3, 1, 2])
    }
}
