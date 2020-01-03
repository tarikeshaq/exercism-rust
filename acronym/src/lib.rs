fn is_space(val: Option<&char>) -> bool {
    match val {
        Some(&inner) => inner == ' ' || inner == '_' || inner == '-',
        None => false
    }
}


pub fn abbreviate(phrase: &str) -> String {
    let p: Vec<char> = phrase.chars().collect();
    let mut res = String::new();
    for (i, c) in phrase.chars().enumerate() {
        if i == 0 || c.is_ascii_uppercase() && p[i-1].is_ascii_lowercase() || is_space(p.get(i-1)) && !is_space(Some(&c)) {
            res.push(c.to_ascii_uppercase())
        }
    }
    res
}
