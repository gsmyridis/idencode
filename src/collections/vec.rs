use std::collections::TryReserveError;

use crate::io::DEFAULT_BUF_SIZE;

#[derive(Default)]
pub struct BitVec {
    inner: Vec<u8>,
    bit_pos: u8,
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
        }
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
        match self.inner.len() {
            0 => 0,
            n => self.bit_pos as usize + 8 * (n - 1)
        }
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` _bytes_.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut vec = vec![1, 2];
    /// vec.push(3);
    /// assert_eq!(vec, [1, 2, 3]);
    /// ```
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
    /// use idencode::BitVec;
    ///
    /// let mut bitvec = BitVec::new();
    /// bitvec.extend_from_slice(&[true, true, false, true, true, false, true, true, true, false]);
    /// assert_eq!(bitvec.as_slice(), &[0b11011011, 0b10]);
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
        self.inner.clear()
    }

    /// Extracts a slice containing the underlying buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let mut bitvec = bitvec![true, true, false];
    /// assert_eq!(*bitvec.as_slice(), [0b110]);
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
    /// assert_eq!(*bitvec.as_slice(), [0b110]);
    /// let slice = bitvec.as_mut_slice();
    /// slice[0] = 0b111;
    /// assert_eq!(slice, [0b111]);
    /// ```
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given `BitVec`. The collection may reserve more space to
    /// speculatively avoid frequent reallocations. After calling `reserve`,
    /// capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` _bytes_.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let mut bitvec = bitvec![true];
    /// bitvec.reserve(10);
    /// assert!(bitvec.capacity() >= 11);
    /// ```
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        let additional = additional_bytes(self.bit_pos, additional);
        if additional > 0 {
            self.inner.reserve(additional);
        }
    }

    /// Tries to reserve capacity for at least `additional` more elements to be inserted
    /// in the given `BitVec`. The collection may reserve more space to speculatively avoid
    /// frequent reallocations. After calling `try_reserve`, capacity will be
    /// greater than or equal to `self.len() + additional` if it returns
    /// `Ok(())`. Does nothing if capacity is already sufficient. This method
    /// preserves the contents even if an error occurs.
    ///
    /// # Errors
    ///
    /// If the capacity overflows, or the allocator reports a failure, then an error
    /// is returned.
    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        let additional = additional_bytes(self.bit_pos, additional);
        if additional > 0 {
            return self.inner.try_reserve(additional);
        }
        Ok(())
    }

    /// Shrinks the capacity of the bit-vector as much as possible.
    ///
    /// The behavior of this method depends on the allocator, which may either shrink the vector
    /// in-place or reallocate. The resulting vector might still have some excess capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let mut bitvec = BitVec::with_capacity(16);
    /// bitvec.extend_from_slice(&[true, true, true]);
    /// assert!(bitvec.capacity() >= 16);
    /// bitvec.shrink_to_fit();
    /// assert!(bitvec.capacity() >= 8);
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
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
// Internal methods and functions
////////////////////////////////////////////////////////////////////////////////

fn additional_bytes(bit_position: u8, additional_bits: usize) -> usize {
    // Available bits left in current byte.
    let left = 8 - (bit_position as usize);
    if additional_bits > left {
        // Additional number of more bits to reserve.
        let additional_bits = additional_bits - left;
        let additional_bytes = (additional_bits + 7) / 8;
        return additional_bytes
    }
    0
}

////////////////////////////////////////////////////////////////////////////////
// Implementation of common traits
////////////////////////////////////////////////////////////////////////////////

impl From<Vec<u8>> for BitVec {
    fn from(v: Vec<u8>) -> BitVec {
        BitVec { inner: v, bit_pos: 0 }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_macro() {
        // Case 1
        let bitvec = bitvec![true; 10];
        assert_eq!(*bitvec.as_slice(), [0b11111111, 0b11]);

        // Case 2 & 3
        let bitvec = bitvec![true, true, false, true, false, ];
        assert_eq!(*bitvec.as_slice(), [0b11010]);
    }

    #[test]
    fn test_len() {
        let bitvec = bitvec![];
        assert_eq!(bitvec.len(), 0);
    }

    #[test]
    fn test_additional_bytes() {
        assert_eq!(additional_bytes(2, 3), 0);
        assert_eq!(additional_bytes(2, 6), 0);
        assert_eq!(additional_bytes(2, 17), 2);
    }
}