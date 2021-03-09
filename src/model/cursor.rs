#[derive(Debug)]
pub struct Cursor {
    column: usize,
    row: usize
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            column: 0,
            row: 0
        }
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn move_right(&mut self) {
        self.column += 1;
    }

    pub fn move_left(&mut self) {
        self.column -= 1;
    }
}

