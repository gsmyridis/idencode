use std::io::{self, Read, Write};

use crate::error::InvalidCodeError;
use crate::num::convert::write_offset_bits;
use crate::num::{bits_to_numeric, Numeric};
use crate::{BitReader, BitWriter};
use crate::{DecodeOne, Decoder, EncodeOne, Encoder};
use crate::{GammaDecoder, GammaEncoder, UnaryDecoder};

/// A structure that wraps a writer and encodes a sequence of integers
/// using Elias Delta Encoding.
///
/// In Elias Delta Encoding, each number if represented by two parts:
/// - The "offset" bits, which are all the binary digits of the number
///   except the leading 1-bit.
///  - The length of the binary representation, encoded in Elias Gamma
///    encoding.
///
/// For example, the number 9 in binary is 1001. Its length is 4 (100),
/// which in Elias gamma encoding is 11000. Therefore, the Elias Delta
/// encoding of 9 is 11000001.
pub struct DeltaEncoder<W> {
    writer: BitWriter<W>,
}

impl<W: Write> DeltaEncoder<W> {
    pub fn new(writer: W) -> Self {
        let writer = BitWriter::new(writer, true);
        DeltaEncoder { writer }
    }
}

impl EncodeOne for DeltaEncoder<()> {
    fn encode_one<T: Numeric>(num: T) -> Vec<bool> {
        let mut offset_bits = vec![];
        write_offset_bits(&num, &mut offset_bits);
        let mut bits = GammaEncoder::encode_one(offset_bits.len() + 1);
        bits.append(&mut offset_bits);
        bits
    }
}

impl<W: Write> Encoder<W> for DeltaEncoder<W> {
    fn encode<T: Numeric>(&mut self, nums: &[T]) -> io::Result<()> {
        let mut offset_bits = Vec::new();

        for n in nums {
            offset_bits.clear();
            write_offset_bits(n, &mut offset_bits);
            let len_bits = GammaEncoder::encode_one(offset_bits.len() + 1);
            self.writer.write_bits(&len_bits)?;
            self.writer.write_bits(&offset_bits)?;
        }
        Ok(())
    }

    fn finalize(self) -> io::Result<W> {
        self.writer.finalize()
    }
}

/// A structure that wraps a reader and decodes a stream of bytes using
/// Elias Delta Encoding.
///
/// In Elias Delta Encoding, each number if represented by two parts:
/// - The "offset" bits, which are all the binary digits of the number
///   except the leading 1-bit.
///  - The length of the binary representation, encoded in Elias Gamma
///    encoding.
///
/// For example, the number 9 in binary is 1001. Its length is 4 (100),
/// which in Elias gamma encoding is 11000. Therefore, the Elias Delta
/// encoding of 9 is 11000001.
pub struct DeltaDecoder<R> {
    reader: BitReader<R>,
}

impl<R: Read> DeltaDecoder<R> {
    pub fn new(reader: R) -> Self {
        let reader = BitReader::new(reader, true);
        DeltaDecoder { reader }
    }
}

impl DecodeOne for DeltaDecoder<()> {
    fn decode_one<T: Numeric>(bits: &[bool]) -> Result<T, InvalidCodeError> {
        let idx = bits
            .iter()
            .position(|b| !b)
            .ok_or_else(|| InvalidCodeError::DeltaCodeError)?;

        let (lb_len_bits, rest) = bits.split_at(idx + 1);
        let len_len_bits = UnaryDecoder::decode_one(&lb_len_bits)?;

        let (offset_len_bits, offset_bits) = rest
            .split_at_checked(len_len_bits)
            .ok_or(InvalidCodeError::DeltaCodeError)?;

        let mut len_bits = Vec::with_capacity(lb_len_bits.len() + offset_len_bits.len());
        len_bits.extend_from_slice(lb_len_bits);
        len_bits.extend_from_slice(offset_len_bits);
        let len = GammaDecoder::decode_one::<usize>(&len_bits)? - 1;

        if offset_bits.len() != len {
            return Err(InvalidCodeError::DeltaCodeError);
        }

        let mut bits = Vec::with_capacity(len);
        bits.push(true);
        bits.extend_from_slice(offset_bits);
        bits_to_numeric::<T>(&bits).or_else(|_| Err(InvalidCodeError::DeltaCodeError))
    }
}

impl<R: Read> Decoder<R> for DeltaDecoder<R> {
    fn decode<T: Numeric>(self) -> Result<Vec<T>, InvalidCodeError> {
        let mut nums = vec![];
        let bitvec = self.reader.read_to_end().expect("Failed to read reader.");
        let bits = bitvec.into_bits();
        let mut current_bits = bits.as_slice();

        while !current_bits.is_empty() {
            let idx = current_bits
                .iter()
                .position(|b| !b)
                .ok_or(InvalidCodeError::DeltaCodeError)?;
            let (unary_bits, rest) = current_bits.split_at(idx + 1);

            let length_of_binary = UnaryDecoder::decode_one(&unary_bits)?;
            if rest.len() < length_of_binary {
                return Err(InvalidCodeError::DeltaCodeError);
            }

            let (binary_bits, rest) = rest.split_at(length_of_binary);
            let mut length_bits = Vec::with_capacity(unary_bits.len() + binary_bits.len());
            length_bits.extend_from_slice(unary_bits);
            length_bits.extend_from_slice(binary_bits);
            let value_length = GammaDecoder::decode_one::<usize>(&length_bits)? - 1;

            if rest.len() < value_length {
                return Err(InvalidCodeError::DeltaCodeError);
            }

            let (value_bits, remaining) = rest.split_at(value_length);

            let mut final_bits = Vec::with_capacity(value_length + 1);
            final_bits.push(true);
            final_bits.extend_from_slice(value_bits);

            let num =
                bits_to_numeric::<T>(&final_bits).map_err(|_| InvalidCodeError::DeltaCodeError)?;

            nums.push(num);
            current_bits = remaining;
        }
        Ok(nums)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encode_one() {
        assert_eq!(
            DeltaEncoder::encode_one(0b10_u8),
            vec![true, false, false, false]
        );
        assert_eq!(
            DeltaEncoder::encode_one(0b11_u8),
            vec![true, false, false, true]
        );
        assert_eq!(
            DeltaEncoder::encode_one(9u8),
            vec![true, true, false, false, false, false, false, true]
        );
    }

    #[test]
    fn test_decode_one() {
        assert_eq!(
            DeltaDecoder::decode_one::<u8>(&[true, false, false, false]),
            Ok(0b10_u8)
        );
        assert_eq!(
            DeltaDecoder::decode_one::<u8>(&[true, false, false, true]),
            Ok(0b11_u8)
        );
        assert_eq!(
            DeltaDecoder::decode_one::<u8>(&[true, true, false, false, false, false, false, true]),
            Ok(9u8)
        );
    }

    #[test]
    fn test_decode_one_errs() {
        assert!(DeltaDecoder::decode_one::<u8>(&[true, false]).is_err());
        assert!(DeltaDecoder::decode_one::<u8>(&[true, false, false]).is_err());
    }

    #[test]
    fn test_encode_decode() {
        // Example 1
        let writer = Cursor::new(vec![]);
        let mut ge = DeltaEncoder::new(writer);
        ge.encode(&[2_u32, 3]).unwrap();
        let result = ge.finalize().unwrap().into_inner();
        assert_eq!(result, vec![0b10001001, 0b10000000]);

        let de = DeltaDecoder::new(Cursor::new(result));
        let nums = de.decode::<u32>().unwrap();
        assert_eq!(nums, vec![2, 3]);

        // Example 2
        let writer = Cursor::new(vec![]);
        let mut ge = DeltaEncoder::new(writer);
        ge.encode(&[2_u32, 3, 9]).unwrap();
        let result = ge.finalize().unwrap().into_inner();
        assert_eq!(result, vec![0b10001001, 0b11000001, 0b10000000]);

        let de = DeltaDecoder::new(Cursor::new(result));
        let nums = de.decode::<u32>().unwrap();
        assert_eq!(nums, vec![2, 3, 9]);
    }
}
