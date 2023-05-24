use bytes::BufMut;

use super::{Block, SIZE_OF_U16};

/// Builds a block.
pub struct BlockBuilder {
    block_size: usize,
    data: Vec<u8>,
    offsets: Vec<u16>,
}

impl BlockBuilder {
    /// Creates a new block builder.
    pub fn new(block_size: usize) -> Self {
        Self {
            block_size,
            data: Vec::new(),
            offsets: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.data.len() + self.offsets.len() * SIZE_OF_U16 + SIZE_OF_U16
    }

    /// Adds a key-value pair to the block. Returns false when the block is full.
    #[must_use]
    pub fn add(&mut self, key: &[u8], value: &[u8]) -> bool {
        assert!(!key.is_empty(), "key must not be empty");
        if !self.data.is_empty()
            && self.len() + key.len() + value.len() + SIZE_OF_U16 * 3 > self.block_size
        {
            return false;
        }
        self.offsets.push(self.data.len() as u16);
        self.data.put_u16(key.len() as u16);
        self.data.put(key);
        self.data.put_u16(value.len() as u16);
        self.data.put(value);
        true
    }

    /// Check if there is no key-value pair in the block.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Finalize the block.
    pub fn build(self) -> Block {
        if self.is_empty() {
            panic!("empty block builder");
        }
        Block {
            data: self.data,
            offsets: self.offsets,
        }
    }
}
