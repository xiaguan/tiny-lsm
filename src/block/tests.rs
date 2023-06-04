use std::sync::Arc;

use super::builder::BlockBuilder;
use super::iterator::BlockIterator;
use super::*;

const TEST_NUM: usize = 1000;

fn generate_key_value() -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut key_value = Vec::new();
    for i in 0..TEST_NUM {
        // If we don't format the key with 6 digits, the key will be like "key1", "key2", ...
        // Then the order of the keys will be wrong.
        // For example, "key10" < "key2" < "key3".
        let key = format!("key{:06}", i);
        let value = format!("value{:06}", i);
        key_value.push((key.into_bytes(), value.into_bytes()));
    }
    key_value
}

fn generate_block() -> Block {
    let key_value = generate_key_value();
    let mut builder = BlockBuilder::new(100000);
    for (key, value) in key_value {
        builder.append(&key, &value);
    }
    builder.build()
}

#[test]
fn test_block() {
    let block = generate_block();
    let mut iter = BlockIterator::new(Arc::new(block));
    // test seek_to()
    for i in 0..TEST_NUM {
        iter.seek_to(i);
        assert_eq!(iter.key(), format!("key{:06}", i).as_bytes());
        assert_eq!(iter.value(), format!("value{:06}", i).as_bytes());
    }
    // test seek_to_key()
    for i in 0..TEST_NUM {
        iter.seek_to_key(format!("key{:06}", i).as_bytes());
        assert_eq!(iter.key(), format!("key{:06}", i).as_bytes());
        assert_eq!(iter.value(), format!("value{:06}", i).as_bytes());
    }
    // test out of range
    iter.seek_to(TEST_NUM);
    assert_eq!(iter.key(), "".as_bytes());
}
