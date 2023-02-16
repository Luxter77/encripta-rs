static TESTS: [(&'static str, &'static str); 1] = [
    ("10.10.70.32", "„yot]ƒ…zp†{" )
]; 

#[test] fn forwards() {
    for (s, d) in TESTS {
        let r: String = super::forward(s);
        std::fs::write("foo.txt", r.clone()).unwrap();
        assert_eq!(r.as_str(), d);
    };
}

#[test] fn backward() {
    for (s, d) in TESTS {
        let r = super::backward(d);
        std::fs::write("foo.txt", r.clone()).unwrap();
        assert_eq!(s, r.as_str());
    };
}

#[test] fn roundtrip() {
    for (s, d) in TESTS {
        let r: String = super::forward(s);
        assert_eq!(super::backward(r.as_str()).as_str(), s);
        let r: String = super::backward(d);
        assert_eq!(super::forward(r.as_str()).as_str(), d);
    };
}