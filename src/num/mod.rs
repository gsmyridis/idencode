mod convert;

use std::fmt::Debug;
use std::ops::{
    BitAnd, BitOrAssign, BitXor, DivAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign,
    Sub, Mul, Add
};

pub use convert::{bits_to_numeric};

/// This trait extends many common integer types (both unsigned and signed)
/// with a few trivial methods so that they can be used
/// with the bitstream handling traits.
pub trait Numeric:
    Sized
    + Copy
    + Default
    + Debug
    + PartialOrd
    + DivAssign
    + Shl<u32, Output = Self>
    + ShlAssign<u32>
    + Shr<u32, Output = Self>
    + ShrAssign<u32>
    + Rem<Self, Output = Self>
    + RemAssign<Self>
    + BitAnd<Self, Output = Self>
    + BitOrAssign<Self>
    + BitXor<Self, Output = Self>
    + Not<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + From<u8>
{
    /// Size of type in bits
    const BITS: u32;

    /// Returns 0 as `Self`.
    const ZERO: Self;

    /// Returns 1 as `Self`.
    const ONE: Self;

    /// Returns the maximum
    const MAX: Self;

    /// Returns true if this value is 0, in its type
    #[inline(always)]
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    /// Assuming 0 <= value < 256, returns this value as a `u8` type
    fn to_u8(self) -> Option<u8>;

    /// Counts the number of leading zeros
    fn leading_zeros(self) -> u32;
}

macro_rules! define_numeric {
    ($t:ty) => {
        impl Numeric for $t {
            const BITS: u32 = <$t>::BITS;
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const MAX: Self = <$t>::MAX;

            #[inline(always)]
            fn to_u8(self) -> Option<u8> {
                if self > <$t>::from(u8::MAX) {
                    None
                } else {
                    Some(self as u8)
                }
            }
            #[inline(always)]
            fn leading_zeros(self) -> u32 {
                <$t>::leading_zeros(self)
            }
        }
    };
}

define_numeric!(u8);
define_numeric!(u16);
define_numeric!(u32);
define_numeric!(u64);
define_numeric!(u128);
