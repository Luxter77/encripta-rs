use std::str::Chars;

const MAGIC:  [u8; 10] = *b"SIAC-UNJBG";
const MAGICL: usize    = MAGIC.len(); // 10


#[cfg(test)] mod test;

pub fn forward(text: &str) -> String {
    let     chars: Chars     = text.chars();
    let mut o:     Vec<char> = Vec::new();
    let mut t:     [u16; 2]  = [0; 2];
    let mut n:     usize     = 0;

    for c in chars {
        c.encode_utf16(&mut t);
        _ = t.iter_mut().map(| x | { *x = (*x + (MAGIC[n % MAGICL] as u16)) % 255; n += 1; });
        o.extend(char::decode_utf16(t).map(| x | { x.unwrap_or('e') }).filter(| x | { *x != '\0'}));
        // n += 1;
    };

    return o.iter().collect();
}

pub fn backward(text: &str) -> String {
    let     chars: Chars     = text.chars();
    let mut o:     Vec<char> = Vec::new();
    let mut t:     [u16; 4]  = [0; 4];
    let mut n:     usize     = 0;

    for c in chars {
        c.encode_utf16(&mut t);
        _ = t.iter_mut().map(| x | { *x = (*x - (MAGIC[n % MAGICL] as u16)) % 255; n += 1; });
        o.extend(char::decode_utf16(t).map(| x | { x.unwrap_or('e') }).filter(| x | { *x != '\0'}));
        // n += 1;
    };

    return o.iter().collect();
}