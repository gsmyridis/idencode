use super::error::InvalidBitError;

/// Turns a bit to a boolean. 1 maps to true, and 0 maps to false.
pub(crate) fn bit_to_bool(b: u8) -> Result<bool, InvalidBitError> {
    match b {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(InvalidBitError),
    }
}

/// Converts a vector of boolean values representing bits (in most significant to
/// least significant order) into its corresponding decimal (integer) representation.
///
/// Each `true` in the vector is treated as `1` and each `false` as `0`, and the vector
/// is interpreted as a binary number. The most significant bit is at the start of the
/// vector, and the least significant bit is at the end.
pub(crate) fn bits_to_number(bits: &[bool]) -> u32 {
    let mut result = 0;
    for (i, &bit) in bits.iter().enumerate() {
        if bit {
            result |= 1 << (bits.len() - 1 - i);
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_to_bool() {
        assert_eq!(bit_to_bool(0u8), Ok(false));
        assert_eq!(bit_to_bool(1u8), Ok(true));
        assert!(bit_to_bool(2u8).is_err());
    }

    #[test]
    fn test_bits_to_num() {
        assert_eq!(bits_to_number(&[true, false]), 2);
        assert_eq!(bits_to_number(&[true, true, false]), 6);
        assert_eq!(bits_to_number(&[false, false, false]), 0);
        assert_eq!(bits_to_number(&[false, false, true]), 1);
    }
}
