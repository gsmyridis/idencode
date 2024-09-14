#[derive(Default)]
pub struct BitQueue {
    inner: Vec<u8>,
    bit_pos: u8,
}

impl From<Vec<u8>> for BitQueue {
    fn from(v: Vec<u8>) -> BitQueue {
        BitQueue { inner: v, bit_pos: 0 }
    }
}

impl BitQueue {
    /// Creates a new bit-queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new bit-queue with specified endianness, and capacity
    /// of the underlying buffer.
    pub fn with_capacity(capacity: usize) -> Self {
        BitQueue {
            inner: Vec::with_capacity(capacity),
            bit_pos: 0,
        }
    }

    /// Pushes a bit at the end of a bit.
    ///
    /// # Example
    ///
    /// ```
    /// use idencode::bitqueue::BitQueue;
    ///
    /// let mut bq = BitQueue::new();
    /// let bits = vec![true, true, false, true, true, false, true, true, true, false];
    /// for bit in bits {
    ///     bq.push(bit);
    /// }
    /// assert_eq!(bq.as_slice(), &[0b11011011, 0b10]);
    /// ```
    pub fn push(&mut self, bit: bool) {
        if self.bit_pos == 0 {
            self.inner.push(0)
        }
        let byte = self.inner.last_mut()
            .expect("It is guaranteed that at least one byte exists.");
        let bit = bit as u8;
        *byte <<= 1;
        *byte |= bit;
        self.bit_pos = (self.bit_pos + 1) % 8;
    }

    /// Extends bit-queue from a slice of bits.
    ///
    /// Traverses the slice of bits in-order and sequentially pushes the bits
    /// in the bit-queue.
    ///
    /// # Example
    ///
    /// ```
    /// use idencode::bitqueue::BitQueue;
    ///
    /// let mut bq = BitQueue::new();
    /// let bits = &[true, true, false, true, true, false, true, true, true, false];
    /// bq.extend(bits);
    ///
    /// assert_eq!(bq.as_slice(), &[0b11011011, 0b10]);
    /// ```
    pub fn extend(&mut self, bits: &[bool]) {
        for bit in bits {
            self.push(*bit);
        }
    }

    /// Returns the current bit position.
    ///
    /// # Example
    ///
    /// ```
    /// use idencode::bitqueue::BitQueue;
    ///
    /// let mut bq = BitQueue::new();
    /// bq.extend(&[true, false]);
    ///
    /// assert_eq!(*bq.bit_position(), 2);
    /// ```
    pub fn bit_position(&self) -> &u8 {
        &self.bit_pos
    }

    /// Returns the number of bits in the bit-queue.
    pub fn n_bits(&self) -> usize {
        self.inner.len() * 8
    }

    /// Returns the number of bytes in the underlying buffer.
    pub fn n_bytes(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the bit-queue contains no bits.
    pub fn is_empty(&self) -> bool {
        self.n_bytes() == 0
    }

    /// Clears the bit-queue, removing all bits.
    ///
    /// Note that this method has no effect on the allocated capacity of the
    /// underlying buffer.
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Extracts a slice containing the underlying buffer.
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    /// Extracts a mutable slice of the underlying buffer.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }

}
