#![allow(clippy::needless_return)] // stilistic choise

use encoding::{Encoding, DecoderTrap, EncoderTrap, all::WINDOWS_1253 as CODEC};

/// magic words to trigger the (default) dark ritual: "SIAC-UNJBG"
// pub const MAGIC:  [u8; 10] = [83, 73, 65, 67, 45, 85, 78, 74, 66, 71]; // "SIAC-UNJBG"
pub const MAGIC:  [u8; 9] = [83, 73, 65, 67, 45, 85, 78, 74, 66]; // "SIAC-UNJB"

// pub fn decypher(_original: &str, _encoded: &str) -> Vec<u8> { Vec::new() }

pub fn forward(text: &str, cypher: &[u8]) -> Result<String, Vec<u8>> {
    let mut o: Vec<u8> = CODEC.encode(text, EncoderTrap::Strict).unwrap();

    for (n, c) in o.iter_mut().enumerate() {
        *c = c.wrapping_add(cypher[n % cypher.len()]);
    };

    return match CODEC.decode(o.as_slice(), DecoderTrap::Strict) {
        Ok(so) => Ok(so),
        Err(_) => Err(o),
    };
}

pub fn backward(text: &str, cypher: &[u8]) -> Result<String, Vec<u8>> {
    let mut o: Vec<u8> = CODEC.encode(text, EncoderTrap::Strict).unwrap();

    for (n, c) in o.iter_mut().enumerate() {
        *c = c.wrapping_sub(cypher[n % cypher.len()]);
    };

    return match CODEC.decode(o.as_slice(), DecoderTrap::Strict) {
        Ok(so) => Ok(so),
        Err(_) => Err(o),
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

