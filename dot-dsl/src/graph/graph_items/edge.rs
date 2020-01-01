use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Edge {
    first: String,
    second: String,
    attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(first: &str, second: &str) -> Edge {
        Edge {
            first: first.to_string(),
            second: second.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Edge {
        attrs.iter().for_each(|(a, b)| {
            self.attrs.insert((*a).to_string(), (*b).to_string());
        });
        self
    }
}
