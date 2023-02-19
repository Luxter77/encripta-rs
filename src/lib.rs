#![allow(clippy::needless_return)] // stilistic choise

use std::borrow::Cow;

use encoding::{Encoding, DecoderTrap, EncoderTrap, all::WINDOWS_1253 as CODEC};

/// magic words to trigger the (default) dark ritual: "SIAC-UNJB"
// pub const MAGIC:  [u8; 10] = [83, 73, 65, 67, 45, 85, 78, 74, 66, 71]; // "SIAC-UNJBG"
pub const MAGIC:  [u8; 9] = [83, 73, 65, 67, 45, 85, 78, 74, 66]; // "SIAC-UNJB"

#[derive(Debug)]
pub enum ShiftError {
    NonRepresentableInput(Cow<'static, str>),
    NonRepresentableOutput((Vec<u8>, Cow<'static, str>)),
    InvalidCypher(Cow<'static, str>),
}

// pub fn decypher(_original: &str, _encoded: &str) -> Result<String, ShiftError> { Vec::new() }

pub fn forward(text: &str, cypher: &[u8]) -> Result<String, ShiftError> {
    let mut o: Vec<u8>;
    if cypher.is_empty() {
        return Err(ShiftError::InvalidCypher(Cow::from("Cypher can't be empty")));
    };
    
    let clen = cypher.len();
    
    match CODEC.encode(text, EncoderTrap::Strict) {
        Ok(s)   => o = s,
        Err(e) => return Err(ShiftError::NonRepresentableInput(e)),
    };

    for (n, c) in o.iter_mut().enumerate() {
        *c = c.wrapping_add(cypher[n % clen]);
    };

    return match CODEC.decode(o.as_slice(), DecoderTrap::Strict) {
        Ok(so) => Ok(so),
        Err(e) => Err(ShiftError::NonRepresentableOutput((o, e))),
    };
}

pub fn backward(text: &str, cypher: &[u8]) -> Result<String, ShiftError> {
    let mut o: Vec<u8>;
    if cypher.is_empty() {
        return Err(ShiftError::InvalidCypher(Cow::from("Cypher can't be empty")));
    };
    
    let clen = cypher.len();

    match CODEC.encode(text, EncoderTrap::Strict) {
        Ok(s)   => o = s,
        Err(e) => return Err(ShiftError::NonRepresentableInput(e)),
    };

    for (n, c) in o.iter_mut().enumerate() {
        *c = c.wrapping_sub(cypher[n % clen]);
    };

    return match CODEC.decode(o.as_slice(), DecoderTrap::Strict) {
        Ok(so) => Ok(so),
        Err(e) => Err(ShiftError::NonRepresentableOutput((o, e))),
    };
}

#[cfg(test)]
mod test {
    static TESTS: [(&'static str, &'static str); 1] = [
        ("10.10.70.32", "„yot]ƒ…zp†{" )
    ]; 

    #[test] fn forwards() {
        for (s, d) in TESTS {
            let r: String = super::forward(s, &super::MAGIC).unwrap();
            std::fs::write("foo.txt", r.clone()).unwrap();
            println!("{s} => {}", r.as_str());
            assert_eq!(r.as_str(), d);
        };
    }

    #[test] fn backward() {
        for (s, d) in TESTS {
            let r = super::backward(d, &super::MAGIC).unwrap();
            std::fs::write("foo.txt", r.clone()).unwrap();
            println!("{d} => {}", r.as_str());
            assert_eq!(s, r.as_str());
        };
    }

    #[test] fn roundtrip() {
        for (s, d) in TESTS {
            let r: String = super::forward(s, &super::MAGIC).unwrap();
            let b: String = super::backward(r.as_str(), &super::MAGIC).unwrap();
            println!("{s} => {} => {}", r.as_str(), b.as_str());
            assert_eq!(b.as_str(), s);

            let r: String = super::backward(d, &super::MAGIC).unwrap();
            let b: String = super::forward(r.as_str(), &super::MAGIC).unwrap();
            println!("{d} => {} => {}", r.as_str(), b.as_str(),);
            assert_eq!(super::forward(r.as_str(), &super::MAGIC).unwrap().as_str(), d);
        };
    }
}

