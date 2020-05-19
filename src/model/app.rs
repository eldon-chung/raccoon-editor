use super::buffer::Buffer;

#[derive(Copy, Clone, PartialEq)]
pub enum AppMode {
	Edit,
	Command,
}

pub struct App {
	buffer: Buffer,
	cursor_position: usize,
	app_mode: AppMode,
}

impl App {
	pub fn new() -> App {
		App {
			buffer: Buffer::new(),
			cursor_position: 0,
			app_mode: AppMode::Edit,
		}
	}

	// App should only release immutable references to the buffer?
	pub fn buffer(&self) -> &Buffer {
		&self.buffer
	}

	pub fn cursor_position(&self) -> usize {
		self.cursor_position
	}

	pub fn app_mode(&self) -> AppMode {
		self.app_mode
	}

	pub fn get_text_as_iter(&self) -> Vec<&str> {
		vec![self.buffer.as_str()]
	}

	pub fn add_char(&mut self, c: char) {
		self.buffer.insert(self.cursor_position, c);
		self.cursor_position += 1;
	}

	pub fn remove_char(&mut self) {
		if self.cursor_position > 1 {
			self.cursor_position -= 1;
			self.buffer.remove(self.cursor_position);
		}
	}
}