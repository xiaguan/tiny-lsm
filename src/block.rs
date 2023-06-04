mod builder;
mod iterator;

/// The size of u16
const U16_SIZE: usize = std::mem::size_of::<u16>();

/// It is made up of key-value pairs and the pair's offset in the block.
/// At the end of the block , there is a 2-byte num that indicates the number of pairs in the block.
/// | key_value paris (stored in data) | offsets (u16 array) | num (2-byte) |
/// |----------------------------------|--------------------|--------------|
/// | key1 | value1 | key2 | value2 | ... | offset1 | offset2 | ... | num |
/// |----------------------------------|--------------------|--------------|
/// | keylen1(u16) | key1 | valuelen1(u16) | value1 | keylen2 | key2 | valuelen2 | value2 | ... | offset1 | offset2 | ... | num |
pub struct Block {
    data: Vec<u8>,
    offsets: Vec<u16>,
}

#[cfg(test)]
mod tests;
