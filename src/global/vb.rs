use crate::error::InvalidVariableByteCode;
use std::ops::DivAssign;

use num_traits::{FromPrimitive, PrimInt};

pub struct VBEncoder;

impl VBEncoder {
    /// Encodes a number in a series of bytes with variable byte (VB) encoding.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::VBEncoder;
    ///
    /// assert_eq!(VBEncoder::encode_one::<u8>(5), vec![0b10000101]);
    /// assert_eq!(VBEncoder::encode_one::<u32>(824), vec![0b00000110, 0b10111000]);
    /// assert_eq!(VBEncoder::encode_one::<u64>(214577), vec![0b00001101, 0b00001100, 0b10110001]);
    ///
    /// ```
    pub fn encode_one<T>(mut n: T) -> Vec<u8>
    where
        T: PrimInt + FromPrimitive + DivAssign,
    {
        let mut bytes = vec![];
        let base = T::from(0x80).expect("base is guaranteed to be u8.");

        loop {
            // Get the 7 bits of the lower byte.
            let byte = (n % base).to_u8().expect("byte is guaranteed to be u8.");
            bytes.insert(0, byte);
            if n < base {
                break;
            }
            // Keep the rest of the bytes.
            n /= base;
        }
        // Add the termination bit for the last byte.
        *bytes
            .last_mut()
            .expect("bytes is guaranteed to not be empty.") |= 0x80;
        bytes
    }

    /// Encodes the
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::VBEncoder;
    ///
    /// assert_eq!(VBEncoder::encode::<u8>(&[5]), vec![0b10000101]);
    /// assert_eq!(VBEncoder::encode::<u32>(&[824, 214577]), vec![0b00000110, 0b10111000, 0b00001101, 0b00001100, 0b10110001]);
    /// ```
    pub fn encode<T>(nums: &[T]) -> Vec<u8>
    where
        T: PrimInt + FromPrimitive + DivAssign,
    {
        let mut encoded = vec![];
        for n in nums {
            let bytes = VBEncoder::encode_one(*n);
            encoded.extend_from_slice(&bytes)
        }
        encoded
    }

    /// Decodes a series of bytes to a number with variable byte (VB) encoding.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::VBEncoder;
    ///
    /// assert_eq!(VBEncoder::decode_one(&[0b10000101]), Ok(5));
    /// assert_eq!(VBEncoder::decode_one(&[0b00000110, 0b10111000]), Ok(824));
    /// assert_eq!(VBEncoder::decode_one(&[0b00001101, 0b00001100, 0b10110001]), Ok(214577));
    /// assert!(VBEncoder::decode_one(&[0b10000011, 0b10101000]).is_err());
    /// ```
    pub fn decode_one(bytes: &[u8]) -> Result<u32, InvalidVariableByteCode> {
        if !VBEncoder::is_valid_code(bytes) {
            return Err(InvalidVariableByteCode);
        }

        let mut n = 0u32;
        for byte in bytes {
            if *byte < 0x80 {
                n = 0x80 * n + (*byte as u32);
            } else {
                n = 0x80 * n + (*byte as u32) - 0x80;
            }
        }
        Ok(n)
    }

    /// Checks if the sequence of bytes represents a well formatted
    /// variable byte code.
    ///
    /// # Examples
    ///
    /// ```
    /// use idencode::VBEncoder;
    ///
    /// assert!(VBEncoder::is_valid_code(&[0b00110001, 0b10100110]));
    /// assert!(!VBEncoder::is_valid_code(&[0b10110001, 0b10100110]));
    /// assert!(!VBEncoder::is_valid_code(&[0b00110001, 0b00100110]));
    /// assert!(!VBEncoder::is_valid_code(&[0b10001100, 0b00110101]));
    /// ```
    pub fn is_valid_code(bytes: &[u8]) -> bool {
        // Count the bytes with termination bit, and keep the position of one.
        // The valid code has only the last byte with a termination bit.
        let mut n_term_bytes = 0u8;
        let mut term_byte_idx = 0u8;
        for (byte_idx, byte) in bytes.iter().enumerate() {
            if *byte > 0x80 {
                n_term_bytes += 1;
                term_byte_idx = byte_idx as u8;
            }

            // If there are more than one byte with terminating bit,
            // the code is invalid.
            if n_term_bytes > 1 {
                return false;
            }
        }

        // If the number of bytes with terminating bytes
        (n_term_bytes == 1) && (term_byte_idx == bytes.len() as u8 - 1)
    }
}
