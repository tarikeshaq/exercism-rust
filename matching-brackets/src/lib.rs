fn opposite(bracket: char) -> char {
    match bracket {
        '}' => '{',
        ')' => '(',
        ']' => '[',
        _ => panic!("Invalid bracket!"),
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in string.trim().chars() {
        match c {
            '{' | '(' | '[' => stack.push(c),
            bracket
                if (bracket == ')' || bracket == '}' || bracket == ']')
                    && !stack.is_empty()
                    && stack.ends_with(&[opposite(bracket)]) =>
            {
                stack.pop();
            }
            bracket
                if (bracket == ')' || bracket == '}' || bracket == ']')
                    && (stack.is_empty() || !stack.ends_with(&[opposite(bracket)])) =>
            {
                return false;
            }
            _ => {}
        }
    }
    stack.is_empty()
}
