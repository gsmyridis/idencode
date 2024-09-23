use std::io::{self, Read, Write};

use crate::code::{Decoder, Encoder};
use crate::error::InvalidCodeError;
use crate::io::read::BitReader;
use crate::io::write::BitWriter;
use crate::num::Numeric;

/// A structure that wraps a writer and encodes a sequence of integers
/// using Variable Byte Encoding.
///
/// Variable byte (VB) encoding uses an integral number of bytes to encode
/// an integer. The last 7 bits of a byte are “payload” and encode part of
/// the integer. The first bit of the byte is a continuation bit. It is set
/// to 1 for the last byte of the encoded gap and to 0 otherwise.
pub struct VBEncoder<W: Write> {
    writer: BitWriter<W>,
}

impl<W: Write> Encoder<W> for VBEncoder<W> {
    fn new(writer: W) -> Self {
        VBEncoder {
            writer: BitWriter::new(writer),
        }
    }

    fn write<T: Numeric>(&mut self, nums: &[T]) -> io::Result<()> {
        let encoded = self.writer.get_mut();
        let base = T::from(0x80_u8);
        let mut num_bytes = vec![];

        for num in nums {
            let mut num = num.to_owned();
            num_bytes.clear();

            loop {
                // Get the 7 bits of the lowest byte.
                let byte = (num % base).to_u8().expect("Guaranteed to be u8.");
                num_bytes.insert(0, byte);
                if num < base {
                    break;
                }
                num /= base; // Keep the rest of the bytes.
            }

            *num_bytes // Add the termination bit for the last byte.
                .last_mut()
                .expect("bytes is guaranteed to not be empty.") += 0x80;

            // Push them to the encoded buffer.
            encoded.extend_from_byte_slice(num_bytes.as_slice());
        }
        Ok(())
    }

    fn finalize(self) -> io::Result<W> {
        self.writer.finalize()
    }
}


/// A structure that wraps a reader and decodes a sequence of integers
/// using Variable Byte Encoding.
///
/// Variable byte (VB) encoding uses an integral number of bytes to encode
/// an integer. The last 7 bits of a byte are “payload” and encode part of
/// the integer. The first bit of the byte is a continuation bit. It is set
/// to 1 for the last byte of the encoded gap and to 0 otherwise.
pub struct VBDecoder<R: Read> {
    reader: BitReader<R>,
}

impl<R: Read> Decoder<R> for VBDecoder<R> {
    fn new(reader: R) -> Self {
        VBDecoder {
            reader: BitReader::new(reader),
        }
    }

    fn decode<T: Numeric>(self) -> Result<Vec<T>, InvalidCodeError> {
        let mut nums = vec![];
        let bitvec = self.reader.read_to_end().unwrap();
        if bitvec.is_empty() {
            return Ok(vec![]);
        }

        let last_byte = *bitvec
            .last_byte()
            .expect("The bitvec is guaranteed to not be empty.");
        if last_byte < 0x80_u8 {
            return Err(InvalidCodeError);
        };

        let mut n = T::ZERO;
        for byte in bitvec.into_bytes() {
            n = T::from(0x80) * n + T::from(byte);
            if byte > 128 {
                n = n - T::from(0x80);
                nums.push(n);
                n = T::ZERO;
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
    fn test_encode_decode_u8() {
        let nums = vec![5, 10, 33];
        let writer = Cursor::new(vec![]);
        let mut vbe = VBEncoder::new(writer);
        vbe.write::<u8>(nums.as_slice()).unwrap();
        let encoded = vbe.finalize().unwrap();
        let encoded = encoded.into_inner();
        assert_eq!(encoded, &[0b10000101, 0b10001010, 0b10100001, 0b10000000]);

        let vbd = VBDecoder::new(Cursor::new(encoded));
        let decoded = vbd.decode::<u8>().unwrap();
        assert_eq!(decoded, nums);
    }

    #[test]
    fn test_encode_decode_u32() {
        let nums = vec![824, 8];
        let writer = Cursor::new(vec![]);
        let mut vbe = VBEncoder::new(writer);
        vbe.write::<u32>(nums.as_slice()).unwrap();
        let encoded = vbe.finalize().unwrap();
        let encoded = encoded.into_inner();
        assert_eq!(encoded, &[0b000000110, 0b10111000, 0b10001000, 0b10000000]);

        let vbd = VBDecoder::new(Cursor::new(encoded));
        let decoded = vbd.decode::<u32>().unwrap();
        assert_eq!(decoded, nums);
    }

    #[test]
    fn test_encode_decode_u64() {
        let nums = vec![214577, 824, 8];
        let writer = Cursor::new(vec![]);
        let mut vbe = VBEncoder::new(writer);
        vbe.write::<u64>(nums.as_slice()).unwrap();
        let encoded = vbe.finalize().unwrap();
        let encoded = encoded.into_inner();
        assert_eq!(
            encoded,
            vec![
                0b00001101,
                0b00001100,
                0b10110001,
                0b000000110,
                0b10111000,
                0b10001000,
                0b10000000
            ]
        );

        let vbd = VBDecoder::new(Cursor::new(encoded));
        let decoded = vbd.decode::<u64>().unwrap();
        assert_eq!(decoded, nums);
    }
}
