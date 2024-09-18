
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
define_error!(InvalidGammaCode, "Invalid gamma code.");
define_error!(InvalidUnaryCode, "Invalid unary code.");

impl From<InvalidUnaryCode> for InvalidGammaCode {
    fn from(_err: InvalidUnaryCode) -> InvalidGammaCode {
        InvalidGammaCode
    }
}
