use std::collections::HashSet;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {
            scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.last() {
            Some(&val) => Some(val),
            None => None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores().iter().max() {
            Some(&val) => Some(val),
            None => None  
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut res = Vec::new();
        let mut seen: HashSet<i32> = HashSet::new();
        for _ in 0..3 {
            let mut max = 0;
            let mut max_idx: i32 = -1;
            for (i, &value) in self.scores().iter().enumerate() {
                if !seen.contains(&(i as i32)) {
                    if value > max {
                        max_idx = i as i32;
                        max = value;
                    }
                }
            }
            if max_idx != -1 {
                seen.insert(max_idx);
                res.push(max);
            }
        }
        res
    }
}
