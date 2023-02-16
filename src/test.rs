use encoding::{Encoding, EncoderTrap, all::WINDOWS_1253 as CODEC, DecoderTrap};

static TESTS: [(&'static str, &'static str); 1] = [
    ("10.10.70.32", "„yot]ƒ…zp†{" )
]; 

#[test] fn forwards() {
    for (s, d) in TESTS {
        let r = super::forward(s);
        let dd = CODEC.encode(d, EncoderTrap::NcrEscape).unwrap();
        std::fs::write("foo.txt", r.clone()).unwrap();
        assert_eq!(r.as_slice(), dd.as_slice());
    };
}

#[test] fn backward() {
    for (s, d) in TESTS {
        let r = super::backward(d);
        std::fs::write("foo.txt", r.clone()).unwrap();
        assert_eq!(s.as_bytes(), r.as_slice());
    };
}

#[test] fn roundtrip() {
    for (s, d) in TESTS {
        let r: String = CODEC.decode(&super::forward(s), encoding::DecoderTrap::Strict).unwrap();
        assert_eq!(CODEC.decode(super::backward(r.as_str()).as_slice(), encoding::DecoderTrap::Strict).unwrap(), s);
        let r: String = CODEC.decode(&super::backward(d), encoding::DecoderTrap::Strict).unwrap();
        assert_eq!(CODEC.decode(super::forward(r.as_str()).as_slice(), encoding::DecoderTrap::Strict).unwrap(), d);
    };
}