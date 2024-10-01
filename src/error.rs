use std::error::Error;
use std::fmt;

/// Creates an error with the provided name and error message.
macro_rules! define_error {
    ($name:ident, $msg:expr) => {
        #[derive(Debug, PartialEq)]
        pub struct $name;

        impl Error for $name {}

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", $msg)
            }
        }
    };
}

define_error!(InvalidVariableByteCode, "Invalid variable byte code.");
define_error!(OverflowError, "Overflow error.");
define_error!(
    BitVecLengthError,
    "The provided length is incompatible with the provided buffer."
);
define_error!(
    NoTerminatingBitError,
    "Did not find a terminating 1-bit in the last byte."
);

#[derive(Debug, PartialEq)]
pub enum InvalidCodeError {
    UnaryCodeError,
    VBCodeError,
    GammaCodeError,
    DeltaCodeError,
}

impl fmt::Display for InvalidCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidCodeError::UnaryCodeError => {
                write!(f, "Invalid Unary Code Error.")
            }
            InvalidCodeError::VBCodeError => {
                write!(f, "Invalid Variable Byte Code Error.")
            }
            InvalidCodeError::GammaCodeError => {
                write!(f, "Invalid Elias Gamma Code Error.")
            }
            InvalidCodeError::DeltaCodeError => {
                write!(f, "Invalid Elias Delta Code Error.")
            }
        }
    }
}

impl Error for InvalidCodeError {}
