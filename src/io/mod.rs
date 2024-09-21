pub mod write;
mod read;

pub use write::BitWriter;

pub const DEFAULT_BUF_SIZE: usize = 1 * 1024;
