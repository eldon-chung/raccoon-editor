use super::buffer::Buffer;
use super::cursor::Cursor;
use super::taggedtext::TaggedText;

#[derive(Copy, Clone, PartialEq)]
pub enum AppMode {
    Edit,
    Command,
}

pub struct App {
    buffer: Buffer,
    app_mode: AppMode,
}

impl App {
    pub fn new() -> App {
        App {
            buffer: Buffer::new(),
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

    pub fn get_tagged_text(&self) -> TaggedText {
        self.buffer.as_tagged_text()
    }

    pub fn add_char(&mut self, c: char) {
        self.buffer.insert(c);
    }

    pub fn remove_char(&mut self) {
        self.buffer.remove();
    }

    pub fn move_cursor_left(&mut self) {
        self.buffer.move_cursor_left();
    }

    pub fn move_cursor_right(&mut self) {
        self.buffer.move_cursor_right();
    }

    pub fn move_cursor_up(&mut self) {
        self.buffer.move_cursor_up();
    }

    pub fn move_cursor_down(&mut self) {
        self.buffer.move_cursor_down();
    }
}
