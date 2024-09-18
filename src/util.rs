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
    fn test_bits_to_num() {
        assert_eq!(bits_to_number(&[true, false]), 0b10);
        assert_eq!(bits_to_number(&[true, true, false]), 0b110);
        assert_eq!(bits_to_number(&[false, false, false]), 0);
        assert_eq!(bits_to_number(&[false, false, true]), 1);
        assert_eq!(
            bits_to_number(&[true, false, false, false, true, true, false, true, true]),
            0b100011011
        );
    }
}
