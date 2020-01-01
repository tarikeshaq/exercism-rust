use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub id: String,
    attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(id: &str) -> Node {
        Node {
            id: id.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Node {
        attrs.iter().for_each(|(a, b)| {
            self.attrs.insert((*a).to_string(), (*b).to_string());
        });
        self
    }

    pub fn get_attr(&self, attr_key: &str) -> Option<&str> {
        match self.attrs.get(attr_key) {
            Some(a) => Some(&a[..]),
            None => None,
        }
    }

    pub fn get_id(self) -> String {
        self.id
    }
}
