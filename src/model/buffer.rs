pub struct Buffer {
    contents: String,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            contents: String::new(),
        }
    }

    pub fn with_string(string: String) -> Buffer {
        Buffer { contents: string }
    }

    pub fn insert(&mut self, idx: usize, ch: char) {
        self.contents.insert(idx, ch);
    }

    pub fn insert_str(&mut self, idx: usize, string: &str) {
        self.contents.insert_str(idx, string);
    }

    pub fn remove(&mut self, idx: usize) -> char {
        self.contents.remove(idx)
    }

    pub fn as_str(&self) -> &str {
        self.contents.as_str()
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }
}
