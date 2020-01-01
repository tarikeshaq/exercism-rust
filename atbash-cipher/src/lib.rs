use itertools::Itertools;

fn cipher_encode(c: char) -> char {
    let pos = c as u8 - b'a';
    let res_pos = 25 - pos;
    (b'a' + res_pos) as char
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut res: String = plain
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_numeric())
        .map(|c| {
            if c.is_alphabetic() {
                cipher_encode(c.to_ascii_lowercase())
            } else {
                c
            }
        })
        .chunks(5)
        .into_iter()
        .flat_map(|chunk| chunk.chain(" ".chars()))
        .collect();
    res.pop();
    res
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
