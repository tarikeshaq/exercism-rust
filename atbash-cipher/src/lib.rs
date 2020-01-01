fn cipher_encode(c: char) -> char {
    let pos = c as u8 - b'a';
    let res_pos = 25 - pos;
    (b'a' + res_pos) as char
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let encoded: String = plain
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_numeric())
        .map(|c| {
            if c.is_alphabetic() {
                cipher_encode(c.to_ascii_lowercase())
            } else {
                c
            }
        })
        .collect();
    let mut spaced = String::new();
    for (i, letter) in encoded.chars().enumerate() {
        if i != 0 && i % 5 == 0 {
            spaced.push_str(" ");
        }
        spaced.push(letter);
    }
    spaced
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_numeric())
        .map(|c| {
            if c.is_alphabetic() {
                cipher_encode(c.to_ascii_lowercase())
            } else {
                c
            }
        })
        .collect()
}
