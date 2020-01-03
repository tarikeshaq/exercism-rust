use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(pt, cs)| {
            cs.iter()
                .flat_map(|c| c.to_lowercase())
                .map(move |c| (c, *pt))
        })
        .collect()
}
