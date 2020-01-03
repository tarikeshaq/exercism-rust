#[derive(Debug, PartialEq)]
pub struct DNA {
    strand: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strand: String,
}

fn valid_dna(c: char) -> bool {
    match c {
        'A' | 'C' | 'T' | 'G' => true,
        _ => false,
    }
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        match dna.chars().enumerate().find(|(_, c)| !valid_dna(*c)) {
            Some((index, _)) => Err(index),
            None => Ok(dna.chars().collect()),
        }
    }

    pub fn into_rna(self) -> RNA {
        let rna: String = self.strand.chars().map(map_rna).collect();
        RNA::new(&rna[..]).unwrap()
    }
}

fn map_rna(c: char) -> char {
    match c {
        'G' => 'C',
        'C' => 'G',
        'T' => 'A',
        'A' => 'U',
        _ => c,
    }
}

fn valid_rna(c: char) -> bool {
    match c {
        'A' | 'C' | 'U' | 'G' => true,
        _ => false,
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        match rna.chars().enumerate().find(|(_, c)| !valid_rna(*c)) {
            Some((index, _)) => Err(index),
            None => Ok(rna.chars().collect()),
        }
    }
}

impl std::iter::FromIterator<char> for DNA {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let strand = String::from_iter(iter);
        DNA { strand }
    }
}

impl std::iter::FromIterator<char> for RNA {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let strand = String::from_iter(iter);
        RNA { strand }
    }
}
