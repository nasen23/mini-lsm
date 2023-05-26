#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

use std::sync::Arc;

use bytes::Buf;

use super::{Block, SIZE_OF_U16};

/// Iterates on a block.
pub struct BlockIterator {
    block: Arc<Block>,
    key: Vec<u8>,
    value: Vec<u8>,
    idx: usize,
}

impl BlockIterator {
    fn new(block: Arc<Block>) -> Self {
        Self {
            block,
            key: Vec::new(),
            value: Vec::new(),
            idx: 0,
        }
    }

    /// Creates a block iterator and seek to the first entry.
    pub fn create_and_seek_to_first(block: Arc<Block>) -> Self {
        let mut iter = Self::new(block);
        iter.seek_to_first();
        iter
    }

    /// Creates a block iterator and seek to the first key that >= `key`.
    pub fn create_and_seek_to_key(block: Arc<Block>, key: &[u8]) -> Self {
        let mut iter = Self::new(block);
        iter.seek_to_key(key);
        iter
    }

    /// Returns the key of the current entry.
    pub fn key(&self) -> &[u8] {
        &self.key
    }

    /// Returns the value of the current entry.
    pub fn value(&self) -> &[u8] {
        &self.value
    }

    /// Returns true if the iterator is valid.
    pub fn is_valid(&self) -> bool {
        self.block.offsets.get(self.idx).is_some()
    }

    /// Seeks to the first key in the block.
    pub fn seek_to_first(&mut self) {
        self.idx = 0;
        self.read_key_value();
    }

    /// Move to the next key in the block.
    pub fn next(&mut self) {
        self.idx += 1;
        self.read_key_value();
    }

    /// Seek to the first key that >= `key`.
    pub fn seek_to_key(&mut self, key: &[u8]) {
        while self.is_valid() {
            self.next();
            if self.key.as_slice() >= key {
                break;
            }
        }
    }

    fn read_key_value(&mut self) {
        if let Some(&offset) = self.block.offsets.get(self.idx) {
            let mut pos = offset as usize;
            let key_size = (&self.block.data[pos..pos + SIZE_OF_U16]).get_u16() as usize;
            pos += SIZE_OF_U16;
            self.key = self.block.data[pos..pos + key_size].to_vec();
            pos += key_size;
            let value_size = (&self.block.data[pos..pos + SIZE_OF_U16]).get_u16() as usize;
            pos += SIZE_OF_U16;
            self.value = self.block.data[pos..pos + value_size].to_vec();
        } else {
            self.key.clear();
            self.value.clear();
        }
    }
}
