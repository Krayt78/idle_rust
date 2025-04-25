use std::fmt;

pub struct Item {
    pub name: String,
    pub description: String,
}

impl Item {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}