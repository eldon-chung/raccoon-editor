pub mod events;

pub enum QuitOption {
	Quitting,
	NotQuitting,
}

#[derive(Debug)]
pub struct Cursor {
	pub node_idx: usize,
	pub node_offset: usize,
	pub line_idx: usize,
	pub line_offset: usize,
}

impl Cursor {
	pub fn new() -> Cursor {
		Cursor {
			node_idx: 0,
			node_offset: 0,
			line_idx: 0,
			line_offset: 0,
		}
	}
}