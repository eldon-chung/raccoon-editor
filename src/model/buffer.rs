use super::cursor::Cursor;

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

    pub fn insert(&mut self, cursor: &Cursor, ch: char) {
        self.contents.insert(cursor.column(), ch);
    }

    pub fn insert_str(&mut self, cursor: &Cursor, string: &str) {
        self.contents.insert_str(cursor.column(), string);
    }

    pub fn remove(&mut self, cursor: &Cursor) -> char {
        self.contents.remove(cursor.column())
    }

    pub fn as_str(&self) -> &str {
        self.contents.as_str()
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }
}
