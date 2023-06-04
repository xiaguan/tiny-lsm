use super::{Block, U16_SIZE};
use bytes::BufMut;

pub struct BlockBuilder {
    data: Vec<u8>,
    offsets: Vec<u16>,
    // The maximum size of the block.
    block_size: usize,
}

impl BlockBuilder {
    pub fn new(block_size: usize) -> Self {
        Self {
            data: Vec::new(),
            offsets: Vec::new(),
            block_size,
        }
    }

    pub fn current_size(&self) -> usize {
        // The size of the data + the size of the offsets (u16) + the size of the num(u16).
        self.data.len() + self.offsets.len() * U16_SIZE + U16_SIZE
    }

    /// Append a key-value pair to the block.
    /// Return true if the pair is successfully appended.
    #[must_use]
    pub fn append(&mut self, key: &[u8], value: &[u8]) -> bool {
        debug_assert!(!key.is_empty(), "Key must not be empty");

        // Why do we need to add U16_SIZE * 3?
        // 1. key_len 2.value_len 3. offfset
        let append_entry_size = key.len() + value.len() + U16_SIZE * 3;
        debug_assert!(
            append_entry_size <= self.block_size,
            "Entry size must not be greater than the block size"
        );

        if self.current_size() + append_entry_size > self.block_size {
            return false;
        }

        self.offsets.push(self.data.len() as u16);
        // key_len | key | value_len | value
        self.data.put_u16(key.len() as u16);
        self.data.put(key);
        self.data.put_u16(value.len() as u16);
        self.data.put(value);
        true
    }

    pub fn build(self) -> Block {
        debug_assert!(!self.offsets.is_empty(), "Block must not be empty");
        Block {
            data: self.data,
            offsets: self.offsets,
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_basic() {
        // crate a block builder with a block size of 20 bytes.
        let mut builder = super::BlockBuilder::new(20);
        // append a key-value pair to the block.
        // after append the block size is ( 4 + 6 + 6  + 2) = 18 bytes.
        assert!(builder.append(b"key1", b"value1"));
        // check the current size of the block.
        assert_eq!(builder.current_size(), 18);
        // append a key-value pair to the block, it will fail because the block size is 20 bytes.
        assert!(!builder.append(b"key2", b"value2"));
    }
}
