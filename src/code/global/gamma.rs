use std::io::{self, Read, Write};

use super::unary::{UnaryDecoder, UnaryEncoder};
use crate::code::{DecodeOne, Decoder, EncodeOne, Encoder};
use crate::error::InvalidCodeError;
use crate::io::read::BitReader;
use crate::io::write::BitWriter;
use crate::num::convert::write_offset_bits;
use crate::num::{bits_to_numeric, Numeric};

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
pub struct GammaEncoder<W> {
    writer: BitWriter<W>,
}

impl<W: Write> GammaEncoder<W> {
    pub fn new(writer: W) -> Self {
        let writer = BitWriter::new(writer, true);
        GammaEncoder { writer }
    }
}

impl EncodeOne for GammaEncoder<()> {
    fn encode_one<T: Numeric>(num: T) -> Vec<bool> {
        let mut offset_bits = vec![];
        write_offset_bits(&num, &mut offset_bits);
        let mut bits = UnaryEncoder::encode_one(offset_bits.len());
        bits.append(&mut offset_bits);
        bits
    }
}

impl<W: Write> Encoder<W> for GammaEncoder<W> {
    fn encode<T: Numeric>(&mut self, nums: &[T]) -> io::Result<()> {
        let mut offset_bits = Vec::new();

        for n in nums {
            offset_bits.clear();
            write_offset_bits(n, &mut offset_bits);
            let len_bits = UnaryEncoder::encode_one(offset_bits.len());
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
pub struct GammaDecoder<R> {
    reader: BitReader<R>,
}

impl<R: Read> GammaDecoder<R> {
    pub fn new(reader: R) -> Self {
        let reader = BitReader::new(reader, true);
        GammaDecoder { reader }
    }
}

impl DecodeOne for GammaDecoder<()> {
    fn decode_one<T: Numeric>(bits: &[bool]) -> Result<T, InvalidCodeError> {
        let idx = bits
            .iter()
            .position(|b| !b)
            .ok_or_else(|| InvalidCodeError::GammaCodeError)?;

        let (len_bits, rest) = bits.split_at(idx + 1);
        let len = UnaryDecoder::decode_one(len_bits)?;

        if rest.len() != len {
            return Err(InvalidCodeError::GammaCodeError);
        }

        let mut n_bits = Vec::with_capacity(len);
        n_bits.push(true);
        n_bits.extend_from_slice(&rest[..len]);

        match bits_to_numeric(n_bits.as_slice()) {
            Ok(num) => Ok(num),
            _ => Err(InvalidCodeError::GammaCodeError),
        }
    }
}

impl<R: Read> Decoder<R> for GammaDecoder<R> {
    fn decode<T: Numeric>(self) -> Result<Vec<T>, InvalidCodeError> {
        let mut nums = vec![];
        let bitvec = self.reader.read_to_end().expect("Failed to read reader.");
        let bits = bitvec.into_bits();
        let mut bits = bits.as_slice();

        while !bits.is_empty() {
            let idx = bits
                .iter()
                .position(|b| !b)
                .ok_or_else(|| InvalidCodeError::GammaCodeError)?;

            let (len_bits, rest) = bits.split_at(idx + 1);
            let len = UnaryDecoder::decode_one(len_bits)?;

            if rest.len() < len {
                return Err(InvalidCodeError::GammaCodeError);
            }

            let mut n_bits = Vec::with_capacity(len);
            n_bits.push(true);
            n_bits.extend_from_slice(&rest[..len]);
            let numeric = bits_to_numeric(n_bits.as_slice()).unwrap();
            nums.push(numeric);

            if let Some((_, r)) = rest.split_at_checked(len) {
                bits = r;
            } else {
                return Err(InvalidCodeError::GammaCodeError);
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
        assert_eq!(GammaEncoder::encode_one(0b10_u32), vec![true, false, false]);
        assert_eq!(GammaEncoder::encode_one(0b11_u32), vec![true, false, true]);
        assert_eq!(
            GammaEncoder::encode_one(9_u32),
            vec![true, true, true, false, false, false, true]
        );
    }

    #[test]
    fn test_encode_decode() {
        // Example 1
        let writer = Cursor::new(vec![]);
        let mut ge = GammaEncoder::new(writer);
        ge.encode(&[2_u32, 3]).unwrap();
        let result = ge.finalize().unwrap().into_inner();
        assert_eq!(result, vec![0b10010110]);

        let de = GammaDecoder::new(Cursor::new(result));
        let nums = de.decode::<u32>().unwrap();
        assert_eq!(nums, vec![2, 3]);

        // Example 2
        let writer = Cursor::new(vec![]);
        let mut ge = GammaEncoder::new(writer);
        ge.encode(&[2_u32, 3, 9]).unwrap();
        let result = ge.finalize().unwrap().into_inner();
        assert_eq!(result, vec![0b10010111, 0b10001100]);

        let de = GammaDecoder::new(Cursor::new(result));
        let nums = de.decode::<u32>().unwrap();
        assert_eq!(nums, vec![2, 3, 9]);
    }

    #[test]
    fn test_decode_errs() {
        let reader = Cursor::new(vec![0b10010111, 0b11100110]);
        let de = GammaDecoder::new(reader);
        assert!(de.decode::<u8>().is_err());

        let reader = Cursor::new(vec![0b11111111]);
        let de = GammaDecoder::new(reader);
        assert!(de.decode::<u8>().is_err());
    }
}
