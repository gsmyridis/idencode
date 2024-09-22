use std::io::{self, Write, Read};

use super::unary::{UnaryDecoder, UnaryEncoder};
use crate::error::InvalidCodeError;
use crate::io::write::BitWriter;
use crate::io::read::BitReader;
use crate::num::{Numeric, bits_to_numeric};
use crate::code::{Encoder, Decoder};

/// A structure that wraps a writer and encodes a sequence of integers
/// using Elias Gamma Encoding.
///
/// In Elias Gamma Encoding, each number is represented by two parts:
/// - The "offset" bits, which are all the binary digits of the number
///   except the leading 1-bit.
/// - The length of the offset, encoded using unary encoding.
///
/// For example, the number 9 in binary is 1001. The offset bits are the
/// remaining digits (001), and the length of these offset bits (3) is
/// encoded in unary as 1110. Therefore, the Elias Gamma encoding of 9
/// is 1110001.
pub struct GammaEncoder<W: Write> {
    writer: BitWriter<W>,
}

impl<W: Write> Encoder<W> for GammaEncoder<W> {
    fn new(writer: W) -> Self {
        GammaEncoder{ writer: BitWriter::new(writer) }
    }

    fn write<T>(&mut self, nums: &[T]) -> io::Result<()>
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

    fn finalize(self) -> io::Result<W> {
        self.writer.finalize()
    }
}


/// A structure that wraps a reader and decodes a stream of bytes
/// using Elias Gamma Encoding.
///
/// In Elias Gamma Encoding, each number is represented by two parts:
/// - The "offset" bits, which are all the binary digits of the number
///   except the leading 1-bit.
/// - The length of the offset, encoded using unary encoding.
///
/// For example, the number 9 in binary is 1001. The offset bits are the
/// remaining digits (001), and the length of these offset bits (3) is
/// encoded in unary as 1110. Therefore, the Elias Gamma encoding of 9
/// is 1110001.
pub struct GammaDecoder<R: Read> {
    reader: BitReader<R>
}

impl<R: Read> Decoder<R> for GammaDecoder<R> {
    fn new(reader: R) -> Self {
        GammaDecoder { reader: BitReader::new(reader) }
    }

    fn decode<T: Numeric>(self) -> Result<Vec<T>, InvalidCodeError> {
        let mut nums = vec![];
        let bitvec = self.reader.read_to_end().expect("Failed to read reader.");
        let bits = bitvec.into_bits();
        let mut bits = bits.as_slice();

        while !bits.is_empty() {
            // Find the first zero (false) bit from the left in bits.
            match bits.iter().position(|b| !b) {
                Some(idx) => {
                    let (len_bits, rest) = bits.split_at(idx + 1);
                    let len = UnaryDecoder::decode(len_bits)?;

                    if rest.len() < len {
                        return Err(InvalidCodeError);
                    }

                    let mut n_bits = Vec::with_capacity(len);
                    n_bits.push(true);
                    n_bits.extend_from_slice(&rest[..len]);
                    let numeric = bits_to_numeric(n_bits.as_slice()).unwrap();
                    nums.push(numeric);

                    if let Some((_, r)) = rest.split_at_checked(len) {
                        bits = r;
                    } else {
                        break;
                    }
                }
                None => return Err(InvalidCodeError),
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
        assert_eq!(result, vec![0b10010000]);

        // Example 2
        let writer = Cursor::new(vec![]);
        let mut ge = GammaEncoder::new(writer);
        ge.write(&[0b11_u32]).unwrap();
        let result = ge.finalize().unwrap().into_inner();
        assert_eq!(result, vec![0b10110000]);

        // Example 3
        let writer = Cursor::new(vec![]);
        let mut ge = GammaEncoder::new(writer);
        ge.write(&[9_u32]).unwrap();
        let result = ge.finalize().unwrap().into_inner();
        assert_eq!(result, vec![0b11100011]);
    }

    #[test]
    fn test_encode_2() {
        // Example 1
        let writer = Cursor::new(vec![]);
        let mut ge = GammaEncoder::new(writer);
        ge.write(&[2_u32, 3]).unwrap();
        let result = ge.finalize().unwrap().into_inner();
        assert_eq!(result, vec![0b10010110]);

        let de = GammaDecoder::new(Cursor::new(result));
        let nums = de.decode::<u32>().unwrap();
        assert_eq!(nums, vec![2, 3]);

        // Example 2
        let writer = Cursor::new(vec![]);
        let mut ge = GammaEncoder::new(writer);
        ge.write(&[2_u32, 3, 9]).unwrap();
        let result = ge.finalize().unwrap().into_inner();
        assert_eq!(result, vec![0b10010111, 0b10001100]);

        let de = GammaDecoder::new(Cursor::new(result));
        let nums = de.decode::<u32>().unwrap();
        assert_eq!(nums, vec![2, 3, 9]);
    }
}
