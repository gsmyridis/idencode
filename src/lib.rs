pub mod code;
pub mod collections;
pub mod error;
pub mod io;
pub mod num;

pub use collections::BitVec;
pub use io::read::BitReader;
pub use io::write::BitWriter;

pub use code::{Encoder, Decoder};
pub use code::global::gamma::{GammaEncoder, GammaDecoder};
pub use code::global::unary::{UnaryDecoder, UnaryEncoder};
pub use code::global::vb::{VBDecoder, VBEncoder};


pub enum Encoding {
    None,
    Gamma,
    VariableByte,
}