use super::Block;
use std::sync::Arc;

use bytes::Buf;

pub struct BlockIterator {
    block: Arc<Block>,
    // idx,key,value are the current key-value pair.
    idx: usize,
    key: Vec<u8>,
    value: Vec<u8>,
}

impl BlockIterator {
    // Create a block iterator from a block.
    // It costs O(1) time.
    pub fn new(block: Arc<Block>) -> Self {
        Self {
            block,
            idx: 0,
            key: Vec::new(),
            value: Vec::new(),
        }
    }

    pub fn key(&self) -> &[u8] {
        &self.key
    }

    pub fn value(&self) -> &[u8] {
        &self.value
    }

    pub fn seek_to(&mut self, idx: usize) {
        if (idx >= self.block.offsets.len()) {
            self.key.clear();
            self.value.clear();
            return;
        }
        // get the offset of the key-value pair.
        let offset = self.block.offsets[idx] as usize;
        self.seek_to_offset(offset);
        self.idx = idx;
    }

    fn seek_to_offset(&mut self, offset: usize) {
        let mut entry = &self.block.data[offset..];
        let key_len = entry.get_u16() as usize;
        self.key = entry[..key_len].to_vec();
        entry.advance(key_len);
        let value_len = entry.get_u16() as usize;
        self.value = entry[..value_len].to_vec();
    }

    pub fn next(&mut self) {
        self.idx += 1;
        self.seek_to(self.idx);
    }

    /// Seek to the first key-value pair whose key is greater than or equal to the given key.
    pub fn seek_to_key(&mut self, key: &[u8]) {
        // We use binary search to find the first key-value pair whose key is greater than or equal to the given key.
        let mut low = 0;
        let mut high = self.block.offsets.len();
        while low < high {
            let mid = low + (high - low) / 2;
            self.seek_to(mid);
            debug_assert!(!self.key().is_empty());
            match self.key().cmp(key) {
                std::cmp::Ordering::Less => {
                    low = mid + 1;
                }
                std::cmp::Ordering::Greater => {
                    high = mid;
                }
                std::cmp::Ordering::Equal => {
                    return;
                }
            }
        }
        self.seek_to(low);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block::builder::BlockBuilder;

    #[test]
    fn test_seek_to() {
        let mut builder = BlockBuilder::new(100);
        assert_eq!(builder.append(b"key1", b"value1"), true);
        assert_eq!(builder.append(b"key2", b"value2"), true);
        assert_eq!(builder.append(b"key3", b"value3"), true);
        let block = builder.build();
        let mut iter = BlockIterator::new(Arc::new(block));
        iter.seek_to(0);
        assert_eq!(iter.key(), b"key1");
        assert_eq!(iter.value(), b"value1");
        iter.seek_to(1);
        assert_eq!(iter.key(), b"key2");
        assert_eq!(iter.value(), b"value2");
        iter.seek_to(2);
        assert_eq!(iter.key(), b"key3");
        assert_eq!(iter.value(), b"value3");
        iter.seek_to(3);
        assert_eq!(iter.key().is_empty(), true);
        assert_eq!(iter.value().is_empty(), true);
    }
}
