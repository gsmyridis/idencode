use crate::error::InvalidUnaryCode;


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
    /// assert_eq!(UnaryEncoder::encode(0), vec![false]);
    /// assert_eq!(UnaryEncoder::encode(1), vec![true, false]);
    /// assert_eq!(UnaryEncoder::encode(2), vec![true, true, false]);
    /// assert_eq!(UnaryEncoder::encode(3), vec![true, true, true, false]);
    /// ```
    pub fn encode(n: usize) -> Vec<bool> {
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
    /// assert_eq!(UnaryDecoder::decode(&[false]), Ok(0));
    /// assert_eq!(UnaryDecoder::decode(&[true, false]), Ok(1));
    /// assert_eq!(UnaryDecoder::decode(&[true, true, false]), Ok(2));
    /// assert!(UnaryDecoder::decode(&[true, true]).is_err());
    /// assert!(UnaryDecoder::decode(&[true, false, true]).is_err());
    /// ```
    pub fn decode(code: &[bool]) -> Result<usize, InvalidUnaryCode> {
        // Check if the code is terminated by '0'.
        if code.last() != Some(&false) {
            return Err(InvalidUnaryCode);
        }

        // Check if the rest of the characters are '1's.
        for c in code[..code.len() - 1].iter() {
            if !(*c) {
                return Err(InvalidUnaryCode);
            }
        }

        Ok(code.len() - 1)
    }
}
