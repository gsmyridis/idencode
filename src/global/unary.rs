use crate::error::InvalidUnaryCode;

pub struct UnaryEncoder;

impl UnaryEncoder {
    /// Encodes a unary encoded number in bits.
    ///
    /// In this code an integer x ≥ 1 is coded as x - 1 one bits followed
    /// by a zero bit, so that the code for integer 3 is 110.
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

    /// Decodes a unary encoded number from bits.
    ///
    /// In this code an integer x ≥ 1 is coded as x - 1 one bits followed
    /// by a zero bit, so that the code for integer 3 is 110.
    ///
    /// # Examples
    /// ```
    /// use idencode::UnaryEncoder;
    ///
    /// assert_eq!(UnaryEncoder::decode(&[false]), Ok(0));
    /// assert_eq!(UnaryEncoder::decode(&[true, false]), Ok(1));
    /// assert_eq!(UnaryEncoder::decode(&[true, true, false]), Ok(2));
    /// assert!(UnaryEncoder::decode(&[true, true]).is_err());
    /// assert!(UnaryEncoder::decode(&[true, false, true]).is_err());
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
