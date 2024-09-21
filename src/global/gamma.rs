use std::io::{self, Write, Read};

use super::unary::{UnaryDecoder, UnaryEncoder};
use crate::error::InvalidGammaCode;
use crate::io::BitWriter;
use crate::num::{Numeric, bits_to_numeric};

pub struct GammaEncoder<W: Write> {
    writer: BitWriter<W>,
}

impl<W: Write> GammaEncoder<W> {
    /// Creates a new Elias gamma encoder, wrapping a writer.
    pub fn new(writer: W) -> Self {
        GammaEncoder{ writer: BitWriter::new(writer) }
    }

    /// Writes numbers in slice in the inner bit-writer after encoding them.
    pub fn write<T>(&mut self, nums: &[T]) -> io::Result<()>
    where
        T: Numeric,
    {
        // We reuse a single vector to store the offset bits for each number in `nums`.
        // For each number, we clear the vector, calculate the offset bits, and store
        // them in the vector. This approach avoids redundant heap allocations by reusing
        // the same vector instead of creating a new one for each number.
        let mut offset_bits = Vec::new();

        for n in nums {
            offset_bits.clear();

            // Calculate the offset bits.
            // We need to find all the bits of the number's binary representation
            // except the leading 1 bit. The way to do this is to extract each bit
            // starting from the most significant bit (after the leading one).
            let leading_one_idx = T::BITS - n.leading_zeros() - 1;
            for i in 0..leading_one_idx {
                let shift = leading_one_idx - i - 1;
                let base = T::ONE << shift;
                if (*n & base).is_zero() {
                    offset_bits.push(false);
                } else {
                    offset_bits.push(true);
                }
            }

            // Encode length of offset bits in unary code.
            let len = offset_bits.len();
            let len_bits = UnaryEncoder::encode(len);

            // Write length and offset bits in bit-writer.
            self.writer.write_bits(&len_bits)?;
            self.writer.write_bits(&offset_bits)?;
        }
        Ok(())
    }

    /// Consumes the encoder and finalizes the encoding, returning the
    /// underlying buffer.
    pub fn finalize(self) -> io::Result<W> {
        self.writer.finalize()
    }
}

pub struct GammaDecoder<R> {
    reader: R
}

impl<R: Read> GammaDecoder<R> {
    /// Creates a new Elias gamma decoder, wrapping a reader.
    pub fn new(reader: R) -> Self {
        GammaDecoder { reader }
    }

    // pub fn decode<T>(mut self) -> Result<Vec<T>, InvalidGammaCode> where T: Numeric {
    //     let mut buffer = vec![];
    //     self.reader.read_to_end(&mut buffer).expect("Failed to read reader.");
    //     Self::decode_bits(buffer.as_ref())
    // }
}

impl GammaDecoder<()> {
    /// Decode a Gamma encoded stream of bits.
    fn decode_bits<T: Numeric>(bits: &[bool]) -> Result<Vec<T>, InvalidGammaCode> {
        let mut nums = vec![];
        let mut bits = bits;

        loop {
            if bits.is_empty() {
                break;
            }

            // Find the first zero bit from the left in bits.
            match bits.iter().position(|b| !b) {
                Some(idx) => {
                    let (len_bits, rest) = bits.split_at(idx + 1);
                    let len = UnaryDecoder::decode(len_bits)?;

                    if rest.len() < len {
                        return Err(InvalidGammaCode);
                    }

                    let mut n_bits = Vec::with_capacity(len);
                    n_bits.push(true);
                    n_bits.extend_from_slice(&rest[..len]);
                    let numeric = bits_to_numeric(n_bits.as_slice()).unwrap();
                    nums.push(numeric);

                    if let Some((_, r)) = rest.split_at_checked(len + 1) {
                        bits = r;
                    } else {
                        break;
                    }
                }
                None => return Err(InvalidGammaCode),
            }
        }
        Ok(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_encode_1() {
        // Example 1
        let writer = Cursor::new(vec![]);
        let mut ge = GammaEncoder::new(writer);
        ge.write(&[0b10_u32]).unwrap();
        let result = ge.finalize().unwrap().into_inner();
        assert_eq!(result, vec![0b10000000]);

        // Example 2
        let writer = Cursor::new(vec![]);
        let mut ge = GammaEncoder::new(writer);
        ge.write(&[0b11_u32]).unwrap();
        let result = ge.finalize().unwrap().into_inner();
        assert_eq!(result, vec![0b10100000]);

        // Example 3
        let writer = Cursor::new(vec![]);
        let mut ge = GammaEncoder::new(writer);
        ge.write(&[9_u32]).unwrap();
        let result = ge.finalize().unwrap().into_inner();
        assert_eq!(result, vec![0b11100010]);
    }

    #[test]
    fn test_encode_2() {
        // Example 1
        let writer = Cursor::new(vec![]);
        let mut ge = GammaEncoder::new(writer);
        ge.write(&[2_u32, 3]).unwrap();
        let result = ge.finalize().unwrap().into_inner();
        assert_eq!(result, vec![0b10010100]);

        // Example 2
        let writer = Cursor::new(vec![]);
        let mut ge = GammaEncoder::new(writer);
        ge.write(&[2_u32, 3, 9]).unwrap();
        let result = ge.finalize().unwrap().into_inner();
        assert_eq!(result, vec![0b10010111, 0b10001000]);
    }

    #[test]
    fn test_decode_bits_success() {
        assert_eq!(GammaDecoder::decode_bits::<u8>(&[]), Ok(vec![]));
        assert_eq!(
            GammaDecoder::decode_bits::<u8>(&[true, false, false]),
            Ok(vec![2])
        );
        assert_eq!(
            GammaDecoder::decode_bits::<u8>(&[true, true, false, false, false]),
            Ok(vec![4])
        );
        assert_eq!(
            GammaDecoder::decode_bits::<u8>(&[true, true, true, false, false, false, true]),
            Ok(vec![9])
        );
    }

    #[test]
    fn test_decode_bits_fail() {
        assert!(GammaDecoder::decode_bits::<u8>(&[false, true, true]).is_err());
        assert!(GammaDecoder::decode_bits::<u8>(&[true, true, false]).is_err());
        assert!(GammaDecoder::decode_bits::<u8>(&[true, true, true]).is_err());
    }

    #[test]
    fn test() {
        let x = Cursor::new(vec![]);
        let mut e = GammaEncoder::new(x);
        e.write(&[u8::MAX, u8::MAX - 1]).unwrap();
        let x = e.finalize().unwrap();

        let d = GammaDecoder::new(x);
        // println!("{:?}", d.decode::<u32>());
    }
}
