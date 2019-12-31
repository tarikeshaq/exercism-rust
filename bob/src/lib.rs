pub fn reply(message: &str) -> &str {
    let mut full = message.to_string();
    full = full.trim().to_string();
    if full.is_empty() {
        return "Fine. Be that way!";
    }
    let s: String = full.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    match (
        !s.is_empty() && s.chars().all(|c| c.is_ascii_uppercase()),
        full.ends_with('?'),
    ) {
        (true, true) => "Calm down, I know what I'm doing!",
        (false, true) => "Sure.",
        (true, false) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}
