use crate::utils::Cursor;

use super::buffer::Buffer;

use std::fs;

#[derive(Copy, Clone, PartialEq)]
pub enum AppMode {
    Edit,
    // IMPORTANT: Change boolean to hold an enum in the future
    // Having difficulties with the traits
    // Write = true, Read = false
    Command(CommandMode),
}

#[derive(Copy, Clone, PartialEq)]
pub enum CommandMode {
    Read,  // opening a file
    Write, // saving a file
}

pub struct App {
    buffer: Buffer,
    command_buffer: Buffer,
    cursor_main: Cursor,
    app_mode: AppMode,
}

impl App {
    pub fn new() -> App {
        App {
            buffer: Buffer::new(),
            command_buffer: Buffer::new(),
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

    pub fn set_app_mode(&mut self, app_mode: AppMode) {
        self.app_mode = app_mode;
    }

    pub fn get_text_as_iter(&self) -> Vec<String> {
        match self.app_mode() {
            AppMode::Edit => vec![self.buffer.as_str()],
            AppMode::Command(_) => vec![self.command_buffer.as_str()],
        }
    }

    pub fn add_char(&mut self, c: char) {
        match self.app_mode() {
            AppMode::Edit => self.buffer.insert(&mut self.cursor_main, c),
            AppMode::Command(_) => self.command_buffer.insert(&mut self.cursor_main, c),
        }
    }

    pub fn remove_char(&mut self) {
        match self.app_mode() {
            AppMode::Edit => self.buffer.remove(&mut self.cursor_main),
            AppMode::Command(_) => self.command_buffer.remove(&mut self.cursor_main),
        }
    }

    pub fn move_cursor_left(&mut self) {
        match self.app_mode() {
            AppMode::Edit => self.buffer.move_cursor_left(&mut self.cursor_main),
            AppMode::Command(_) => self.command_buffer.move_cursor_left(&mut self.cursor_main),
        }
    }

    pub fn move_cursor_right(&mut self) {
        match self.app_mode() {
            AppMode::Edit => self.buffer.move_cursor_right(&mut self.cursor_main),
            AppMode::Command(_) => self.command_buffer.move_cursor_right(&mut self.cursor_main),
        }
    }

    pub fn save_file(&mut self) {
        assert!(self.app_mode() == AppMode::Command(CommandMode::Write));

        // Get from the command_buffer
        let filename = self.get_text_as_iter().join("");

        // Get from the normal buffer
        self.set_app_mode(AppMode::Edit);
        let text_to_save = self.get_text_as_iter().join("");

        fs::write(filename, text_to_save).expect("Unable to write file");
    }

    pub fn open_file(&mut self) {
        assert!(self.app_mode() == AppMode::Command(CommandMode::Read));

        // Get from the command_buffer
        let filename = self.get_text_as_iter().join("");

        // Get contents from file, and initialise buffer with those contents
        let data = fs::read_to_string(filename).expect("Unable to read file");
        self.buffer = Buffer::with_contents(data);

        // Enter editing mode after this
        self.set_app_mode(AppMode::Edit);
    }
}

#[cfg(test)]
#[path = "tests/app_tests.rs"]
mod app_tests;
