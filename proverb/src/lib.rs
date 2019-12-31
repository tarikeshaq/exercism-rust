pub fn build_proverb(list: &[&str]) -> String {
    let mut res = String::new();
    for (i, word) in list.iter().enumerate() {
        if i != list.len() - 1 {
            res.push_str(&format!("For want of a {} the {} was lost.\n", word, list[i+1]));
        }
    }
    if list.len() != 0 {
        res.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    res
}
