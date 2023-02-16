#![allow(clippy::needless_return)] // stilistic choise

use encoding::{Encoding, DecoderTrap, EncoderTrap, all::WINDOWS_1253 as CODEC};

/// magic words to trigger the dark ritual: "SIAC-UNJBG"
const MAGIC:  [u8; 10] = [83, 73, 65, 67, 45, 85, 78, 74, 66, 71]; // "SIAC-UNJBG"
/// 9usize
const MAGICL: usize    = MAGIC.len() - 1; // I don't know

#[cfg(test)] mod test;

// pub fn decypher(_original: &str, _encoded: &str) -> Vec<u8> { Vec::new() }

pub fn forward(text: &str) -> String {
    let mut o: Vec<u8> = CODEC.encode(text, EncoderTrap::Strict).unwrap();

    for (n, c) in o.iter_mut().enumerate() {
        *c = c.wrapping_add(MAGIC[n % MAGICL]);
    };

    println!("{}", CODEC.decode(o.as_ref(), DecoderTrap::Strict).unwrap());

    return CODEC.decode(o.as_slice(), encoding::DecoderTrap::Strict).unwrap();
}

pub fn backward(text: &str) -> String {
    let mut o: Vec<u8> = CODEC.encode(text, EncoderTrap::Strict).unwrap();

    for (n, c) in o.iter_mut().enumerate() {
        *c = c.wrapping_sub(MAGIC[n % MAGICL]);
    };

    println!("{}", CODEC.decode(o.as_ref(), DecoderTrap::Strict).unwrap());

    return CODEC.decode(o.as_slice(), encoding::DecoderTrap::Strict).unwrap();

}