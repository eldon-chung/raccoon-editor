use super::buffer::Buffer;
use super::taggedtext::TaggedText;

use std::fs;
use std::fs::File;
use std::io::ErrorKind;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AppMode {
    Edit,
    // IMPORTANT: Change boolean to hold an enum in the future
    // Having difficulties with the traits
    // Write = true, Read = false
    Command(CommandMode),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CommandMode {
    Read,  // opening a file
    Write, // saving a file
}

pub struct App {
    buffer: Buffer,
    command_buffer: Buffer,
    mode: AppMode,
}

impl App {
    pub fn new(args: &[String]) -> App {
        let (buffer, command_buffer) = if args.len() < 2 {
            // No extra arguments passed.
            // args[0] will always be the name of our binary
            (Buffer::new(), Buffer::new())
        } else {
            // Some arguments are passed in the command line. Just take the first one
            let contents = App::read_file_content(args[1].clone());
            (
                Buffer::with_contents(contents),
                Buffer::with_contents(args[1].clone()),
            )
        };

        App {
            buffer: buffer,
            command_buffer: command_buffer,
            mode: AppMode::Edit,
        }
    }

    // App should only release immutable references to the buffer?
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn mode(&self) -> AppMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: AppMode) {
        self.mode = mode;
    }

    pub fn get_command_buffer_as_tagged_text(&self) -> TaggedText {
        self.command_buffer.as_tagged_text()
    }

    pub fn get_buffer_as_tagged_text(&self) -> TaggedText {
        self.buffer.as_tagged_text()
    }

    pub fn get_buffer_text(&self) -> String {
        self.buffer.as_str()
    }

    pub fn get_command_buffer_text(&self) -> String {
        self.command_buffer.as_str()
    }

    pub fn get_text_as_iter(&self) -> Vec<String> {
        vec![self.buffer.as_str()]
    }

    // Originally implemented with get_text_as_iter using a match depending on the mode
    // but decided to separate it out and write another method because accessing a buffer
    // should be independent of mode.
    // We can discuss the naming of this method if required
    pub fn get_command_buffer_text_as_iter(&self) -> Vec<String> {
        vec![self.command_buffer.as_str()]
    }

    // This method was written to make the View as "dumb" as possible
    // The App will handle which text is to be shown
    pub fn get_text_based_on_mode(&self) -> Vec<String> {
        match self.mode() {
            AppMode::Edit => self.get_text_as_iter(),
            AppMode::Command(_) => self.get_command_buffer_text_as_iter(),
        }
    }

    pub fn add_char(&mut self, c: char) {
        match self.mode() {
            AppMode::Edit => self.buffer.insert(c),
            AppMode::Command(_) => self.command_buffer.insert(c),
        }
    }

    pub fn remove_char(&mut self) {
        match self.mode() {
            AppMode::Edit => self.buffer.remove(),
            AppMode::Command(_) => self.command_buffer.remove(),
        }
    }

    pub fn move_cursor_left(&mut self) {
        match self.mode() {
            AppMode::Edit => self.buffer.move_cursor_left(),
            AppMode::Command(_) => self.command_buffer.move_cursor_left(),
        }
    }

    pub fn move_cursor_right(&mut self) {
        match self.mode() {
            AppMode::Edit => self.buffer.move_cursor_right(),
            AppMode::Command(_) => self.command_buffer.move_cursor_right(),
        }
    }

    pub fn move_cursor_up(&mut self) {
        self.buffer.move_cursor_up();
    }

    pub fn move_cursor_down(&mut self) {
        self.buffer.move_cursor_down();
    }

    pub fn save_file(&mut self) {
        assert_eq!(self.mode(), AppMode::Command(CommandMode::Write));

        let file_path = self.get_command_buffer_text();

        // Get from normal buffer
        let text_to_save = self.get_text_as_iter().join("");

        fs::write(file_path, text_to_save).expect("Unable to write file");

        // Back to editing mode after saving
        self.set_mode(AppMode::Edit);
    }

    pub fn open_file(&mut self) {
        assert_eq!(self.mode(), AppMode::Command(CommandMode::Read));

        let file_path = self.get_command_buffer_text();

        // Get contents from file, and initialise buffer with those contents
        let contents = App::read_file_content(file_path);
        self.buffer = Buffer::with_contents(contents);

        // Enter editing mode after this
        self.set_mode(AppMode::Edit);
    }

    fn read_file_content(file_path: String) -> String {
        let contents = match fs::read_to_string(&file_path) {
            Ok(contents) => contents,
            Err(ref e) if e.kind() == ErrorKind::NotFound => {
                App::init_new_file(file_path);
                String::new()
            }
            Err(e) => panic!("{}", e),
        };
        contents
    }

    pub fn handle_regular_save(&mut self) {
        // Probably better to store this somewhere aside from relying on command_buffer
        let file_path = self.get_command_buffer_text();

        if file_path.len() == 0 {
            // Empty string, so most likely a fresh text
            self.set_mode(AppMode::Command(CommandMode::Write));
        } else {
            // There is a path. Save there directly

            // A bit of constraint right now is that to save file, we should be in the CommandMode::Write mode
            // Implemented initially for safety to minimise logical bugs of saving in non-write mode
            // But that means we have to artificially enter the mode, before we can save
            self.set_mode(AppMode::Command(CommandMode::Write));
            self.save_file();

            // Save File should bring you back to Edit Mode
            assert_eq!(self.mode(), AppMode::Edit);
        }
    }

    pub fn handle_save_as_new_file(&mut self) {
        // Just enter the command mode write normally if we want to save as new file
        self.set_mode(AppMode::Command(CommandMode::Write));
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
