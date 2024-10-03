use crate::error::BitVecLengthError;
use crate::io::DEFAULT_BUF_SIZE;

#[derive(Debug, Clone)]
pub struct BitVec {
    inner: Vec<u8>,
    bit_pos: u8,
    len: usize,
}

impl BitVec {
    /// Creates a new bit-vector with specified buffer of bytes and length of
    /// bit-vector.
    ///
    /// The number of bits in the buffer is necessary because the buffer will
    /// always contain bytes.
    ///
    /// # Errors
    ///
    /// If the number of bits in the buffer is more than the capacity or there
    /// are bytes that unnecessary.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::BitVec;
    ///
    /// let bitvec = BitVec::with_len(vec![0b10011001, 0b10001000], 14).unwrap();
    /// assert_eq!(*bitvec.as_bytes(), [0b10011001, 0b10001000]);
    /// assert_eq!(bitvec.len(), 14);
    /// assert_eq!(*bitvec.bit_position(), 6);
    ///
    /// assert!(BitVec::with_len(vec![1, 2, 3], 15).is_err());
    /// assert!(BitVec::with_len(vec![1, 2, 3], 25).is_err());
    /// ```
    pub fn with_len(buf: Vec<u8>, len: usize) -> Result<Self, BitVecLengthError> {
        if (len > 8 * buf.len()) | (len < 8 * (buf.len() - 1)) {
            return Err(BitVecLengthError);
        }
        Ok(BitVec {
            inner: buf,
            bit_pos: (len % 8) as u8,
            len,
        })
    }

    /// Constructs a new `BitVec` from a buffer of bits. The number of bits
    /// is a multiple of 8.
    pub fn new(buffer: Vec<u8>) -> Self {
        let len = buffer.len() * 8;
        BitVec {
            inner: buffer,
            bit_pos: 0,
            len,
        }
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
            inner: Vec::with_capacity((capacity + 7) / 8),
            bit_pos: 0,
            len: 0,
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
    /// let mut bq = BitVec::default();
    /// let bits = vec![true, true, false, true, true, false, true, true, true, false];
    /// for bit in bits {
    ///     bq.push(bit);
    /// }
    /// assert_eq!(bq.as_bytes(), &[0b11011011, 0b10000000]);
    /// ```
    pub fn push(&mut self, bit: bool) {
        if self.bit_pos == 0 {
            self.inner.push(0)
        }
        let byte = self
            .inner
            .last_mut()
            .expect("It is guaranteed that at least one byte exists.");
        *byte |= (bit as u8) << 7 - self.bit_pos;
        self.bit_pos = (self.bit_pos + 1) % 8;
        self.len += 1;
    }

    /// Pushes a whole byte to the underlying buffer of bytes.
    ///
    /// Note that if the current bit has not been filled, it will be padded with
    /// 0-bits.
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let mut bitvec = bitvec![true, true, false];
    /// bitvec.push_byte(0b10000000);
    /// assert_eq!(*bitvec.as_bytes(), [0b11000000, 0b10000000]);
    /// ```
    pub fn push_byte(&mut self, byte: u8) {
        self.inner.push(byte);
        self.bit_pos = 0;
    }

    /// Pushes whole bytes to the underlying buffer of bytes.
    ///
    /// Note that if the current bit has not been filled, it will be padded with
    /// 0-bits.
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let mut bitvec = bitvec![true, true, false];
    /// bitvec.extend_from_byte_slice(&[0b10000000, 0b10000000]);
    /// assert_eq!(*bitvec.as_bytes(), [0b11000000, 0b10000000, 0b10000000]);
    /// ```
    #[inline]
    pub fn extend_from_byte_slice(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.inner.push(*byte);
        }
    }

    /// Inserts an element at position `index` within the vector, shifting all
    /// elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let mut bitvec = bitvec![true, true, false, true];
    /// bitvec.insert_byte(0, 0b10101010);
    /// assert_eq!(*bitvec.as_bytes(), [0b10101010, 0b11010000]);
    /// ```
    ///
    /// # Time complexity
    ///
    /// Takes *O*([`Vec::len`]) time. All items after the insertion index must be
    /// shifted to the right. In the worst case, all elements are shifted when
    /// the insertion index is 0.
    #[inline]
    pub fn insert_byte(&mut self, index: usize, byte: u8) {
        self.inner.insert(index, byte)
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
    /// let mut bitvec = BitVec::default();
    /// bitvec.extend_from_slice(&[true, true, false, true, true, false, true, true, true, false]);
    /// assert_eq!(bitvec.as_bytes(), &[0b11011011, 0b10000000]);
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

    /// Extracts a shared reference to the last byte.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let bitvec = bitvec![true; 11];
    /// let last_byte = bitvec.last_byte().unwrap();
    /// assert_eq!(*last_byte, 0b11100000);
    /// ```
    pub fn last_byte(&self) -> Option<&u8> {
        self.inner.last()
    }

    /// Extracts a mutable reference to the last byte.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let mut bitvec = bitvec![true; 11];
    /// let last_byte = bitvec.last_byte_mut().unwrap();
    /// *last_byte = 0;
    /// assert_eq!(*bitvec.as_bytes(), [0b11111111, 0]);
    /// ```
    #[inline]
    pub fn last_byte_mut(&mut self) -> Option<&mut u8> {
        self.inner.last_mut()
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
    /// assert_eq!(*bitvec.as_bytes(), [0b11000000]);
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
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
    /// assert_eq!(*bitvec.as_bytes(), [0b11000000]);
    /// let slice = bitvec.as_bytes_mut();
    /// slice[0] = 0b111;
    /// assert_eq!(slice, [0b111]);
    /// ```
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }

    /// Consumes the bit-vector returning the underlying buffer of bytes.
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let bitvec = bitvec![true, true, false];
    /// assert_eq!(bitvec.into_bytes(), vec![0b11000000]);
    /// ```
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.inner
    }

    /// Converts the bit-vector to a vector of bits in boolean form.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::{BitVec, bitvec};
    ///
    /// let bitvec = bitvec![true, false, true, false, true, true, false, true, true, true];
    /// let bits = bitvec.into_bits();
    /// assert_eq!(bits, vec![true, false, true, false, true, true, false, true, true, true]);
    /// ```
    pub fn into_bits(self) -> Vec<bool> {
        let mut bits = vec![];
        for i in 0..self.len() {
            let byte = self.inner.get(i / 8).expect("Guaranteed to get byte.");
            let bit_pos = i % 8;
            let bit = byte & (1 << (7 - bit_pos));
            match bit {
                0 => bits.push(false),
                _ => bits.push(true),
            }
        }
        bits
    }
}

////////////////////////////////////////////////////////////////////////////////
// Macros
////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! bitvec {
    ($bit:expr; $n:expr) => {{
        let mut bitvec = BitVec::default();
        bitvec.extend_from_slice(&[$bit; $n]);
        bitvec
    }};
    ( $( $b:expr ),* ) => {{
        let mut bitvec = BitVec::default();
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

impl Default for BitVec {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_BUF_SIZE)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_macro() {
        // Case 1
        let bitvec = bitvec![true; 10];
        assert_eq!(*bitvec.as_bytes(), [0b11111111, 0b11000000]);

        // Case 2 & 3
        let bitvec = bitvec![true, true, false, true, false,];
        assert_eq!(*bitvec.as_bytes(), [0b11010000]);
    }

    #[test]
    fn test_len() {
        let bitvec = bitvec![];
        assert_eq!(bitvec.len(), 0);
    }
}
