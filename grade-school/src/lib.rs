use std::collections::HashMap;

pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.grades.entry(grade).or_insert_with(Vec::new);
        (*students).push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut res = self.grades.keys().copied().collect::<Vec<u32>>();
        res.sort();
        res
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades.get(&grade).map(|c| {
            let mut cloned = c.clone();
            cloned.sort();
            cloned
        })
    }
}
