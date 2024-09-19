use std::io::{Result, Read};


pub struct BitReader<R: Read> {
    inner: R
}

impl<R: Read> BitReader<R> {
    pub fn new(reader: R) -> Self {
        BitReader{ inner: reader }
    }

    // pub fn read_to_end(mut self) -> Result<Vec<bool>> {
    //     let mut buffer = vec![];
    //     self.inner.read_to_end(&mut buffer)?;
    //
    //
    // }

}