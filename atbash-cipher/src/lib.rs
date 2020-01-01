fn cipher_encode(c: char) -> char {
    let pos = c as u8 - b'a';
    let res_pos = 25 - pos;
    (b'a' + res_pos) as char
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_numeric())
        .map(|c| {
            if c.is_alphabetic() {
                cipher_encode(c.to_ascii_lowercase())
            } else {
                c
            }
        })
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % 5 == 0 {
                std::iter::once(Some(' ')).chain(std::iter::once(Some(c)))
            } else {
                std::iter::once(Some(c)).chain(std::iter::once(None))
            }
        })
        .filter_map(|opt| opt)
        .collect()
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
