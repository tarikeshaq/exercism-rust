use std::collections::HashMap;

fn not_valid(nucleotide: char) -> bool {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => false,
        _ => true,
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if not_valid(nucleotide) {
        Err(nucleotide)
    } else {
        dna.chars().try_fold(0, |mut res, c| {
            if not_valid(c) {
                Err(c)
            } else {
                if c == nucleotide {
                    res += 1;
                }
                Ok(res)
            }
        })
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    ['A', 'C', 'G', 'T']
        .iter()
        .try_fold(HashMap::new(), |mut map, c| {
            map.insert(*c, count(*c, dna)?);
            Ok(map)
        })
}
