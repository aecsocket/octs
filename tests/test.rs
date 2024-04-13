use std::num::NonZeroU64;

use bytes::BytesMut;
use octs::{Read, VarInt, Write};

#[test]
fn run() {
    let mut bytes = BytesMut::new();
    bytes.write(&50u32).unwrap();
    bytes.write(&20u32).unwrap();
    bytes.write(&75u64).unwrap();
    bytes.write(&0u64).unwrap();
    bytes.write(&VarInt(50u32)).unwrap();
    println!("{}", bytes.len());

    let mut bytes = bytes.freeze();
    println!("{:?}", bytes.read::<u32>());
    println!("{:?}", bytes.read::<u32>());
    println!("{:?}", bytes.read::<NonZeroU64>());
    println!("{:?}", bytes.read::<NonZeroU64>());
}
