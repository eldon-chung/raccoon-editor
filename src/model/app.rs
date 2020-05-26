use crate::utils::cursor::Cursor;

use super::buffer::Buffer;

#[derive(Copy, Clone, PartialEq)]
pub enum AppMode {
    Edit,
    Command,
}

pub struct App {
    buffer: Buffer,
    cursor_main: Cursor,
    app_mode: AppMode,
}

impl App {
    pub fn new() -> App {
        App {
            buffer: Buffer::new(),
            cursor_main: Cursor::new(),
            app_mode: AppMode::Edit,
        }
    }

    // App should only release immutable references to the buffer?
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn cursor_main(&self) -> &Cursor {
        &self.cursor_main
    }

    pub fn app_mode(&self) -> AppMode {
        self.app_mode
    }

    pub fn get_text_as_iter(&self) -> Vec<String> {
        vec![self.buffer.as_str()]
    }

    pub fn add_char(&mut self, c: char) {
        self.buffer.insert(&mut self.cursor_main, c);
    }

    pub fn remove_char(&mut self) {
        self.buffer.remove(&mut self.cursor_main);
    }

    pub fn move_cursor_left(&mut self) {
        self.buffer.move_cursor_left(&mut self.cursor_main);
    }

    pub fn move_cursor_right(&mut self) {
        self.buffer.move_cursor_right(&mut self.cursor_main);
    }
}
