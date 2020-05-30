use crate::utils::Cursor;

use super::buffer::Buffer;

use std::fs;
use std::fs::File;
use std::io::ErrorKind;

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
        vec![self.buffer.as_str()]
    }

    // Originally implemented with get_text_as_iter using a match depending on the app_mode
    // but decided to separate it out and write another method because accessing a buffer
    // should be independent of app_mode.
    // We can discuss the naming of this method if required
    pub fn get_command_buffer_text_as_iter(&self) -> Vec<String> {
        vec![self.command_buffer.as_str()]
    }

    // This method was written to make the View as "dumb" as possible
    // The App will handle which text is to be shown
    pub fn get_text_based_on_mode(&self) -> Vec<String> {
        match self.app_mode() {
            AppMode::Edit => self.get_text_as_iter(),
            AppMode::Command(_) => self.get_command_buffer_text_as_iter(),
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

        let file_path = self.get_command_buffer_text_as_iter().join("");

        // Get from normal buffer
        let text_to_save = self.get_text_as_iter().join("");

        fs::write(file_path, text_to_save).expect("Unable to write file");

        // The current implementation will quit after saving. This is here
        // for the time when we are going to implement saving without quitting
        self.set_app_mode(AppMode::Edit);
    }

    pub fn open_file(&mut self) {
        assert!(self.app_mode() == AppMode::Command(CommandMode::Read));

        let file_path = self.get_command_buffer_text_as_iter().join("");

        // Get contents from file, and initialise buffer with those contents
        let contents = match fs::read_to_string(&file_path) {
            Ok(contents) => contents,
            Err(ref e) if e.kind() == ErrorKind::NotFound => {
                App::init_new_file(file_path);
                String::new()
            }
            Err(e) => panic!("{}", e),
        };
        self.buffer = Buffer::with_contents(contents);

        // Reset cursor to the start of file
        self.cursor_main = Cursor::new();

        // Enter editing mode after this
        self.set_app_mode(AppMode::Edit);
    }

    fn init_new_file(filepath: String) {
        let _file = match File::create(filepath) {
            Err(e) => panic!("{}", e),
            Ok(f) => f,
        };
    }
}

#[cfg(test)]
#[path = "tests/app_tests.rs"]
mod app_tests;
