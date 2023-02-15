
static TESTS: [(&'static str, &'static str); 1] = [
    ("10.10.70.32", "â€žyot]Æ’â€¦zpâ€ {" /* "„yot]ƒ…zp†{" */)
]; 

#[test] fn forwards() {
    for (s, d) in TESTS { 
        println!("assert> {s} => {d}");
        println!("assert> {s:?} => {d:?}", s=s.as_bytes(), d=d.as_bytes());
        
        let r = super::forward(s);
        
        println!("ress>>> {s:?} => {r:?}", s=s.as_bytes(), r=r.as_bytes());
        assert_eq!(r.as_str(), s)
    };
}

#[test] fn backward() {
    for (s, d) in TESTS {
        println!("assert> {d} => {s}");
        println!("assert> {d:?} => {s:?}", d=d.as_bytes(), s=s.as_bytes());
        
        let r = super::backward(d);
        
        println!("ress>>> {d:?} => {r:?}", d=d.as_bytes(), r=r.as_bytes());
        assert_eq!(d, r.as_str())
    };
}
