use super::unary::UnaryEncoder;
use crate::error::InvalidGammaCode;
use crate::util::bits_to_number;
use crate::write::BitWriter;

#[derive(Default)]
pub struct GammaEncoder {
    writer: BitWriter,
}

impl GammaEncoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&mut self, nums: &[u32]) {
        // We reuse a single vector to store the offset bits for each number in `nums`.
        // For each number, we clear the vector, calculate the offset bits, and store
        // them in the vector. This approach avoids redundant heap allocations by reusing
        // the same vector instead of creating a new one for each number.
        let mut offset_bits = vec![];

        for n in nums {
            offset_bits.clear();

            // Calculate the offset bits.
            // We need to find all the bits of the number's binary representation
            // except the leading 1 bit. The way to do this is to `AndBit` each
            // power of 2, with the number itself.
            let leading_one_idx = u32::BITS - n.leading_zeros() - 1;
            for i in 0..leading_one_idx {
                let base = 1 << (leading_one_idx - i - 1);
                if n & base == 0 {
                    offset_bits.push(false);
                } else {
                    offset_bits.push(true);
                }
            }

            // Encode length of offset bits in unary code.
            let len = offset_bits.len();
            let len_bits = UnaryEncoder::encode(len);

            // Write length and offset bits in bit-writer.
            self.writer.write_bits(len_bits.as_slice());
            self.writer.write_bits(offset_bits.as_slice());
        }
    }

    /// Consumes the encoder and finalizes the encoding, returning the
    /// underlying buffer.
    pub fn finalize(self) -> Vec<u8> {
        self.writer.finalize()
    }
}

pub struct GammaDecoder;

impl GammaDecoder {
    /// Decode a Gamma encoded stream of bits.
    fn decode_bits(bits: &[bool]) -> Result<Vec<u32>, InvalidGammaCode> {
        let mut nums = vec![];
        let mut bits = bits;

        loop {
            // If the bits is empty, break.
            if bits.is_empty() {
                break;
            }

            // Find the first zero bit from the left in bits.
            match bits.iter().position(|b| !b) {
                Some(idx) => {
                    let (len_bits, rest) = bits.split_at(idx + 1);
                    let len = UnaryEncoder::decode(len_bits)?;

                    if rest.len() < len {
                        return Err(InvalidGammaCode);
                    }

                    let mut n_bits = Vec::with_capacity(len);
                    n_bits.push(true);
                    n_bits.extend_from_slice(&rest[..len]);
                    nums.push(bits_to_number(n_bits.as_slice()));

                    if let Some((_, r)) = rest.split_at_checked(len + 1) {
                        bits = r;
                    } else {
                        break;
                    }
                }
                None => return Err(InvalidGammaCode),
            }
        }

        Ok(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_1() {
        // Example 1
        let mut ge = GammaEncoder::new();
        ge.write(&[2]);
        assert_eq!(ge.finalize(), vec![4]);

        // Example 2
        let mut ge = GammaEncoder::new();
        ge.write(&[3]);
        assert_eq!(ge.finalize(), vec![5]);

        // Example 3
        let mut ge = GammaEncoder::new();
        ge.write(&[9]);
        assert_eq!(ge.finalize(), vec![113]);
    }

    #[test]
    fn test_encode_2() {
        // Example 1
        let mut ge = GammaEncoder::new();
        ge.write(&[2, 3]);
        assert_eq!(ge.finalize(), vec![37]);

        // Example 2
        let mut ge = GammaEncoder::new();
        ge.write(&[2, 3, 9]);
        assert_eq!(ge.finalize(), vec![151, 17]);
    }

    #[test]
    fn test_decode_bits_success() {
        assert_eq!(GammaDecoder::decode_bits(&[]), Ok(vec![]));
        assert_eq!(
            GammaDecoder::decode_bits(&[true, false, false]),
            Ok(vec![2])
        );
        assert_eq!(
            GammaDecoder::decode_bits(&[true, true, false, false, false]),
            Ok(vec![4])
        );
        assert_eq!(
            GammaDecoder::decode_bits(&[true, true, true, false, false, false, true]),
            Ok(vec![9])
        );
    }

    #[test]
    fn test_decode_bits_fail() {
        assert!(GammaDecoder::decode_bits(&[false, true, true]).is_err());
        assert!(GammaDecoder::decode_bits(&[true, true, false]).is_err());
        assert!(GammaDecoder::decode_bits(&[true, true, true]).is_err());
    }
}
