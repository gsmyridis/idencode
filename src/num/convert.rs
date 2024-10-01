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

/// Calculate the offset bits.
///
/// We need to find all the bits of the number's binary representation
/// except the leading 1 bit. The way to do this is to extract each bit
/// starting from the most significant bit (after the leading one).
pub(crate) fn write_offset_bits<T: Numeric>(num: &T, buffer: &mut Vec<bool>) {
    let leading_one_idx = T::BITS - num.leading_zeros() - 1;
    for i in 0..leading_one_idx {
        let shift = leading_one_idx - i - 1;
        let base = T::ONE << shift;
        if (*num & base).is_zero() {
            buffer.push(false);
        } else {
            buffer.push(true);
        }
    }
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
