use std::collections::{HashMap, HashSet};

fn initilize_map(word: &str) -> HashMap<char, i32> {
    let mut characters_map = HashMap::new();
    word.chars().flat_map(|c| c.to_lowercase()).for_each(|c| {
        let count = characters_map.entry(c).or_insert(0);
        *count += 1;
    });
    characters_map
}

fn is_anagram(word: &str, candidate: &str) -> bool {
    if candidate
        .chars()
        .flat_map(|c| c.to_lowercase())
        .eq(word.chars().flat_map(|c| c.to_lowercase()))
    {
        return false;
    }

    let mut characters_map = initilize_map(word);
    candidate
        .chars()
        .flat_map(|c| c.to_lowercase())
        .for_each(|c| {
            let count = characters_map.entry(c).or_insert(0);
            *count -= 1;
        });
    characters_map.iter().all(|(_, value)| *value == 0)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|candidate| is_anagram(word, candidate))
        .copied()
        .collect()
}
