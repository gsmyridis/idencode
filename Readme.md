# Integer Encoding-Decoding

`idencode` is a Rust library designed to efficiently decode and encode a stream of integers into bits. It currently
supports the following encoding schemes:
- Unary (`UnaryEncoder`, `UnaryDecoder`)
- Variable Byte (`VBEncoder`, `VBDecoder`)
- Elias Gamma (`GammaEncoder`, `GammaDecoder`)
- Elias Delta (`DeltaEncoder`, `DeltaDecoder`)
 
Additional encoding schemes are planned for future releases, and the library's infrastructure is designed to simplify 
the process of adding them. The primary purpose of creating the library was to learn about the encodings and the
language.

## Key Components
The library’s core components are:

- `BitVec`
- `BitWriter`
- `BitReader`
- Traits: `Encode`, `EncodeOne`, `Decode`, `DecodeOne`, and `Numeric`

### `BitVec`
`BitVec` is the fundamental structure that stores individual bits. Bits are stored using bit-endian byte order and
most-significant-bit first bit order, effectively treating the BitVec as a standard vector of bits. While future
enhancements may introduce options for custom byte or bit orderings, the current design is sufficient for encoding
purposes. 

Additionally, there is a `bitvec!` macro, that works like `vec!`, for quick creation of `BitVec`s.

### `BitWriter`
`BitWriter` wraps around a Write and allows writing a stream of bits. Internally, it uses a BitVec as a buffer, which
is then written to the Write instance when finalized.

If specified with `term_bit = true`, when finalizing the writing process, an additional terminating bit (1) is appended to signal the end
of the bitstream. This is essential because computers store data in whole bytes, not individual bits. For example,
if you write three bits (`true`, `true`, `false`), you’ll have `0b11000000` in the byte. However, when reading this
back, there's no way to tell how many bits were originally written without a terminating marker. Therefore, `BitWriter`
writes `0b11010000`, ensuring the bitstream ends with a 1. This behavior will be customizable in future releases,
and if your data naturally aligns with whole bytes (i.e., bit lengths that are multiples of 8), the terminating bit
can be omitted.

### `BitReader`
`BitReader` wraps a `Read` and reads a stream of bits from it, storing the result in a `BitVec`. The BitVec is
returned, making it easy to retrieve and interpret the bits. Similar to `BitWriter`, if the `BitReader` is instantiated
with `term_bit = true`, the reader will look for the last, terminating bit; otherwise, it will read a multiple of 8
number of bits (all the bits in the bytes).

### `Encode`, `EncodeOne` and `Decode`, `DecodeOne`
The `Encode` and `Decode` traits are implemented by various encoders and decoders. These traits define the behavior
for serializing and deserializing integers to a `Write` and from a `Read`. On the other hand, `EncodeOne`, and 
`DecodeOne`, encode and decode only a single number into a `Vec<bool>` and from `&[bool]`.

### `Numeric`
`Numeric` is a custom trait implemented by all unsigned integer types that can be encoded and decoded by the library. 
This abstraction allows flexibility in applying encoding and decoding strategies to different numeric types.


## Planned Features
`idencode` is not under active development, but will likely improve in the future. Specifically:

- Refactor for maintainability
- Improvement of API
- Performance optimizations
- Better error handling (remove `anyhow`)
- Improve documentation
- Support for additional encoding schemes
- Enhanced customization for byte and bit orderings (`BitQueue`)
