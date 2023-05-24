#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

mod builder;
mod iterator;

const SIZE_OF_U16: usize = std::mem::size_of::<u16>();

pub use builder::BlockBuilder;
use bytes::{BufMut, Bytes};
pub use iterator::BlockIterator;

/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted
/// key-value pairs.
pub struct Block {
    data: Vec<u8>,
    offsets: Vec<u16>,
}

impl Block {
    pub fn encode(&self) -> Bytes {
        let mut buf = Vec::<u8>::with_capacity(
            self.data.len() + self.offsets.len() * SIZE_OF_U16 + SIZE_OF_U16,
        );
        buf.put(self.data.as_slice());
        for &offset in self.offsets.iter() {
            buf.put_u16(offset);
        }
        buf.put_u16(self.offsets.len() as u16);
        buf.into()
    }

    pub fn decode(data: &[u8]) -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests;
