use crate::error::InvalidCodeError;

/// A structure that encodes a non-negative integer using unary encoding.
///
/// In this version of unary encoding, a number *n* is represented by *n*
/// consecutive 1-bits followed by a terminating 0-bit.
///
/// For example, the number 3 is encoded as 1110 in unary.
pub struct UnaryEncoder;

impl UnaryEncoder {
    /// Encodes a unary encoded number in bits.
    ///
    /// # Examples
    /// ```
    /// use idencode::UnaryEncoder;
    ///
    /// assert_eq!(UnaryEncoder::encode_one(0), vec![false]);
    /// assert_eq!(UnaryEncoder::encode_one(1), vec![true, false]);
    /// assert_eq!(UnaryEncoder::encode_one(2), vec![true, true, false]);
    /// assert_eq!(UnaryEncoder::encode_one(3), vec![true, true, true, false]);
    /// ```
    pub fn encode_one(n: usize) -> Vec<bool> {
        let mut bits = Vec::with_capacity(n + 1);
        bits.extend(vec![true; n]);
        bits.push(false);
        bits
    }
}

/// A structure that decodes a stream of bits using unary encoding.
///
/// In this version of unary encoding, a number *n* is represented by *n*
/// consecutive 1-bits followed by a terminating 0-bit.
///
/// For example, the number 3 is encoded as 1110 in unary.
pub struct UnaryDecoder;

impl UnaryDecoder {
    /// Decodes a unary encoded number from bits.
    ///
    /// # Examples
    /// ```
    /// use idencode::UnaryDecoder;
    ///
    /// assert_eq!(UnaryDecoder::decode_one(&[false]), Ok(0));
    /// assert_eq!(UnaryDecoder::decode_one(&[true, false]), Ok(1));
    /// assert_eq!(UnaryDecoder::decode_one(&[true, true, false]), Ok(2));
    /// assert!(UnaryDecoder::decode_one(&[true, true]).is_err());
    /// assert!(UnaryDecoder::decode_one(&[true, false, true]).is_err());
    /// ```
    pub fn decode_one(code: &[bool]) -> Result<usize, InvalidCodeError> {
        // Check if the code is terminated by '0'.
        if code.last() != Some(&false) {
            return Err(InvalidCodeError::UnaryCodeError);
        }

        // Check if the rest of the characters are '1's.
        for c in code[..code.len() - 1].iter() {
            if !(*c) {
                return Err(InvalidCodeError::UnaryCodeError);
            }
        }

        Ok(code.len() - 1)
    }
}
