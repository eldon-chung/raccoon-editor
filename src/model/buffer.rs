use super::cursor::Cursor;

pub struct Buffer {
    contents: Vec<Vec<char>>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            contents: vec![Vec::new()],
        }
    }

    fn string_to_vec(string: String) -> Vec<char> {
        string.chars().collect()
    }

    pub fn with_string(string: String) -> Buffer {
        Buffer {
            contents: vec![string.chars().collect()],
        }
    }

    pub fn insert(&mut self, cursor: &Cursor, ch: char) {
        self.contents[cursor.row()].insert(cursor.column(), ch);
    }

    pub fn remove(&mut self, cursor: &Cursor) -> char {
        self.contents[cursor.row()].remove(cursor.column())
    }

    pub fn as_str(&self) -> String {
        self.contents.iter().fold(String::new(), |mut acc, s| {
            acc.push_str(&s.iter().cloned().collect::<String>());
            acc
        })
    }

    pub fn num_lines(&self) -> usize {
        self.contents.len()
    }
}
