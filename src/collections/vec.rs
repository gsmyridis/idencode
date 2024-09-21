use crate::io::DEFAULT_BUF_SIZE;

#[derive(Default)]
pub struct BitVec {
    inner: Vec<u8>,
    bit_pos: u8,
    len: usize,
}

impl BitVec {
    /// Constructs a new, empty `BitVec`.
    ///
    /// The `BitVec` does not allocate until bits are pushed into it.
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_BUF_SIZE)
    }

    /// Constructs a new, empty `BitVec` with at least the specified capacity.
    ///
    /// The vector will be able to hold at least `capacity` bits without
    /// reallocating. This method is allowed to allocate for more bits than
    /// `capacity`. If `capacity` is 0, the bit-queue will not allocate.
    ///
    /// It is important to note that although the returned bit-vector has the
    /// minimum *capacity* specified, the bit-vector will have a zero *length*.
    ///
    /// If it is important to know the exact allocated capacity of a `BitVec`,
    /// always use the `capacity` method after construction.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` _bytes_.
     ///
    /// # Examples
    ///
    /// ```
    /// use idencode::BitVec;
    /// let mut vec = BitVec::with_capacity(16);
    ///
    /// // The vector contains no items, even though it has capacity for more
    /// assert_eq!(vec.len(), 0);
    /// assert!(vec.capacity() >= 16);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        BitVec {
            inner: Vec::with_capacity((capacity + 7) / 8 ),
            bit_pos: 0,
            len: 0
        }
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` _bytes_.
    ///
    /// # Time complexity
    ///
    /// Takes amortized *O*(1) time. If the vector's length would exceed its
    /// capacity after the push, *O*(*capacity*) time is taken to copy the
    /// vector's elements to a larger allocation. This expensive operation is
    /// offset by the *capacity* *O*(1) insertions it allows.
    ///
    /// # Example
    ///
    /// ```
    /// use idencode::collections::BitVec;
    ///
    /// let mut bq = BitVec::new();
    /// let bits = vec![true, true, false, true, true, false, true, true, true, false];
    /// for bit in bits {
    ///     bq.push(bit);
    /// }
    /// assert_eq!(bq.as_slice(), &[0b11011011, 0b10000000]);
    /// ```
    pub fn push(&mut self, bit: bool) {
        if self.bit_pos == 0 {
            self.inner.push(0)
        }
        let byte = self.inner.last_mut()
            .expect("It is guaranteed that at least one byte exists.");
        *byte |= (bit as u8) << 7 - self.bit_pos;
        self.bit_pos = (self.bit_pos + 1) % 8;
        self.len += 1;
    }

    /// Extends bit-queue from a slice of bits.
    ///
    /// Traverses the slice of bits in-order and sequentially pushes the bits
    /// in the bit-queue.
    ///
    /// # Example
    ///
    /// ```
    /// use idencode::BitVec;
    ///
    /// let mut bitvec = BitVec::new();
    /// bitvec.extend_from_slice(&[true, true, false, true, true, false, true, true, true, false]);
    /// assert_eq!(bitvec.as_slice(), &[0b11011011, 0b10000000]);
    /// ```
    #[inline]
    pub fn extend_from_slice(&mut self, bits: &[bool]) {
        for bit in bits {
            self.push(*bit);
        }
    }

    /// Returns the current bit position.
    ///
    /// # Example
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let bq = bitvec![true, false];
    /// assert_eq!(*bq.bit_position(), 2);
    /// ```
    #[inline]
    pub fn bit_position(&self) -> &u8 {
        &self.bit_pos
    }

    /// Returns the total number of elements the bit-vector can hold without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::BitVec;
    /// let mut vec = BitVec::with_capacity(10);
    /// vec.push(true);
    /// assert!(vec.capacity() >= 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity() * 8
    }

    /// Returns the number of bits in the bit-vector, also referred to
    /// as its 'length'.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let bitvec = bitvec![true; 11];
    /// assert_eq!(bitvec.len(), 11);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the number of bytes in the underlying buffer.
    ///
    /// # Example
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let bitvec = bitvec![false; 27];
    /// assert_eq!(bitvec.n_bytes(), 4);
    /// ```
    #[inline]
    pub fn n_bytes(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the bit-vector contains no bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let mut bitvec = bitvec![true, true, false];
    /// bitvec.clear();
    /// assert!(bitvec.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }

    /// Clears the bit-vector, removing all bits.
    ///
    /// Note that this method has no effect on the allocated capacity of the
    /// underlying buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let mut bitvec = bitvec![true, true, false];
    /// assert_eq!(bitvec.len(), 3);
    /// bitvec.clear();
    /// assert_eq!(bitvec.len(), 0);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
        self.len = 0;
    }

    /// Extracts a slice containing the underlying buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let mut bitvec = bitvec![true, true, false];
    /// assert_eq!(*bitvec.as_slice(), [0b11000000]);
    /// ```
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    /// Extracts a mutable slice of the inner underlying buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let mut bitvec = bitvec![true, true, false];
    /// assert_eq!(*bitvec.as_slice(), [0b11000000]);
    /// let slice = bitvec.as_mut_slice();
    /// slice[0] = 0b111;
    /// assert_eq!(slice, [0b111]);
    /// ```
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Internal methods and functions
    ////////////////////////////////////////////////////////////////////////////////

    fn additional_bytes(&self, additional_bits: usize) -> usize {
        // Available bits left in current byte.
        let left = 8 - (self.bit_pos as usize);
        if additional_bits > left {
            // Additional number of more bits to reserve.
            let additional_bits = additional_bits - left;
            let additional_bytes = (additional_bits + 7) / 8;
            return additional_bytes
        }
        0
    }
}

////////////////////////////////////////////////////////////////////////////////
// Macros
////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! bitvec {
    ($bit:expr; $n:expr) => {{
        let mut bitvec = BitVec::new();
        bitvec.extend_from_slice(&[$bit; $n]);
        bitvec
    }};
    ( $( $b:expr ),* ) => {{
        let mut bitvec = BitVec::new();
        bitvec.extend_from_slice(&[$( $b ),* ]);
        bitvec
    }};
    ( $( $b:expr ),+ ,) => {
        bitvec![ $( $b ), *]
    }
}

////////////////////////////////////////////////////////////////////////////////
// Implementation of common traits
////////////////////////////////////////////////////////////////////////////////

impl From<Vec<u8>> for BitVec {
    fn from(v: Vec<u8>) -> BitVec {
        BitVec { inner: v, bit_pos: 0, len: 0 }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_macro() {
        // Case 1
        let bitvec = bitvec![true; 10];
        assert_eq!(*bitvec.as_slice(), [0b11111111, 0b11000000]);

        // Case 2 & 3
        let bitvec = bitvec![true, true, false, true, false, ];
        assert_eq!(*bitvec.as_slice(), [0b11010000]);
    }

    #[test]
    fn test_len() {
        let bitvec = bitvec![];
        assert_eq!(bitvec.len(), 0);
    }

    #[test]
    fn test_additional_bytes() {
        let bitvec = bitvec![true, false];
        assert_eq!(bitvec.additional_bytes(3), 0);
        assert_eq!(bitvec.additional_bytes(6), 0);
        assert_eq!(bitvec.additional_bytes(17), 2);
    }
}