use crate::error::OverflowError;
use crate::num::Numeric;

/// Converts a vector of boolean values representing bits (in most significant to
/// least-significant order) into its corresponding decimal (integer) representation.
///
/// Each `true` in the vector is treated as `1` and each `false` as `0`, and the vector
/// is interpreted as a binary number. The most significant bit is at the start of the
/// vector, and the least significant bit is at the end.
pub fn bits_to_numeric<T: Numeric>(bits: &[bool]) -> Result<T, OverflowError> {
    if bits.len() > (T::BITS - 1) as usize {
        return Err(OverflowError);
    }
    let mut result = T::ZERO;
    for (i, &bit) in bits.iter().enumerate() {
        if bit {
            let shift = u32::try_from(bits.len() - 1 - i)
                .expect("It is guaranteed that the length of the bits does not exceed u32::MAX.");
            result |= T::ONE << shift;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_num() {
        assert_eq!(bits_to_numeric::<u8>(&[true, false]), Ok(0b10));
        assert_eq!(bits_to_numeric::<u8>(&[true, true, false]), Ok(0b110));
        assert_eq!(bits_to_numeric::<u8>(&[false, false, false]), Ok(0));
        assert_eq!(bits_to_numeric::<u32>(&[false, false, true]), Ok(1));
        let nums = &[true, false, false, false, true, true, false, true, true];
        assert_eq!(bits_to_numeric::<u32>(nums), Ok(0b100011011));
        assert!(bits_to_numeric::<u8>(nums).is_err());
    }
}
