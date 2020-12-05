#[derive(Debug)]
pub struct Cursor {
    column: usize,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor { column: 0 }
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn move_right(&mut self) {
        self.column += 1;
    }

    pub fn move_left(&mut self) {
        self.column -= 1;
    }
}
