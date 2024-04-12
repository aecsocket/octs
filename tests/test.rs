use std::num::NonZeroU64;

use bytes::BytesMut;
use octs::Write;

#[test]
fn run() {
    let mut bytes = BytesMut::new();
    bytes.write(&50).unwrap();
    bytes.write(&20).unwrap();
    println!("{}", bytes.len());
}
