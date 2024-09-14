
/// Creates an error with the provided name and error message.
macro_rules! create_error {
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

create_error!(InvalidBitError, "Bit can be either 1 or 0.");
create_error!(InvalidVariableByteCode, "Invalid variable byte code.");
create_error!(InvalidGammaCode, "Invalid gamma code.");
create_error!(InvalidUnaryCode, "Invalid unary code.");

impl From<InvalidUnaryCode> for InvalidGammaCode {
    fn from(_err: InvalidUnaryCode) -> InvalidGammaCode {
        InvalidGammaCode
    }
}
