use crate::bitqueue::BitQueue;

/// This structure represents a bit-writer.
#[derive(Default)]
pub struct BitWriter {
    buffer: BitQueue
}

impl BitWriter {
    /// Creates a new bit-writer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new bit-writer with underlying buffer with specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        BitWriter { buffer: BitQueue::with_capacity(capacity) }
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
    /// use idencode::io::BitWriter;
    ///
    /// let mut bw = BitWriter::default();
    /// let bits = vec![true, true, false, true, false, false, false, false];
    /// for bit in bits {
    ///     bw.write_bit(bit);
    /// }
    ///
    /// assert_eq!(*bw.get_ref(), [0b11010000]);
    /// ```
    pub fn write_bit(&mut self, bit: bool) {
        self.buffer.push(bit);
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
        self.buffer.clear()
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
    /// use idencode::io::BitWriter;
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
        self.buffer.as_slice()
            .iter()
            .copied()
            .collect::<Vec<_>>()
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
