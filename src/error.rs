/// Creates an error with the provided name and error message.
macro_rules! define_error {
    ($name:ident, $msg:expr) => {
        #[derive(Debug, PartialEq)]
        pub struct $name;

        impl std::error::Error for $name {}

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", $msg)
            }
        }
    };
}

define_error!(InvalidVariableByteCode, "Invalid variable byte code.");
define_error!(InvalidCodeError, "Invalid code.");
define_error!(OverflowError, "Overflow error.");
define_error!(
    BitVecLengthError,
    "The provided length is incompatible with the provided buffer."
);
define_error!(
    NoTerminatingBitError,
    "Did not find a terminating 1-bit in the last byte."
);
