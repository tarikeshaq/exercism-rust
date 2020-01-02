use std::collections::{HashMap, HashSet};

fn is_anagram(word: &str, candidate: &str) -> bool {
    let mut candidate_iter = candidate.chars().flat_map(|c| c.to_lowercase());
    let word_iter = word.chars().flat_map(|c| c.to_lowercase());
    if candidate_iter.clone().eq(word_iter.clone()) {
        return false;
    }

    let mut characters_map = HashMap::new();
    word_iter.for_each(|c| {
        let count = characters_map.entry(c).or_insert(0);
        *count += 1;
    });

    match candidate_iter.try_for_each(|c| {
        let count = characters_map.entry(c).or_insert(0);
        *count -= 1;
        if *count < 0 {
            Err(())
        } else {
            Ok(())
        }
    }) {
        Err(_) => false,
        Ok(_) => characters_map.iter().all(|(_, value)| *value == 0),
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|candidate| is_anagram(word, candidate))
        .copied()
        .collect()
}
