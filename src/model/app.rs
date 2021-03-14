use super::buffer::Buffer;
use super::cursor::Cursor;

#[derive(Copy, Clone, PartialEq)]
pub enum AppMode {
    Edit,
    Command,
}

pub struct App {
    buffer: Buffer,
    cursor: Cursor,
    app_mode: AppMode,
}

impl App {
    pub fn new() -> App {
        App {
            buffer: Buffer::new(),
            cursor: Cursor::new(),
            app_mode: AppMode::Edit,
        }
    }

    // App should only release immutable references to the buffer?
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn app_mode(&self) -> AppMode {
        self.app_mode
    }

    pub fn get_text_as_iter(&self) -> Vec<String> {
        vec![self.buffer.as_str()]
    }

    pub fn add_char(&mut self, c: char) {
        self.buffer.insert(&self.cursor, c);
        self.cursor.move_right();
    }

    pub fn remove_char(&mut self) {
        if self.cursor.column() > 1 {
            self.cursor.move_left();
            self.buffer.remove(&self.cursor);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor.column() > 0 {
            self.cursor.move_left();
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor.column() < self.buffer.len_at_col(&self.cursor) - 1 {
            self.cursor.move_right();
        }
    }
}
