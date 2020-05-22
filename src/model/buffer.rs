use crate::utils::Cursor;
#[allow(dead_code)]


#[derive(Copy, Clone)]
enum BufferType {
	Original,
	Added,
}

#[derive(Clone)]
struct BufferNode {
	from: BufferType,
	index: usize,
	offset: usize,
	line_offsets: Vec<usize>,
}

impl BufferNode {
	fn new(from: BufferType, index: usize, offset: usize, line_offsets: Vec<usize>) -> BufferNode {
		BufferNode{
			from,
			index,
			offset,
			line_offsets,
		}
	}

	fn index(&self) -> usize {
		self.index
	}

	fn offset(&self) -> usize {
		self.offset
	}

	fn from(&self) -> BufferType {
		self.from
	}

	fn line_offsets(&mut self) -> &mut Vec<usize> {
		&mut self.line_offsets
	}
}

pub struct Buffer {
	original_str: String,
	added_str: String,
	node_list: Vec<BufferNode>,
}

impl Buffer {
	fn get_offsets(string: &str) -> Vec<usize> {
		// Whelp this looks gross. Probably prettify or rewrite later on
		let acc = vec![0];
		let index_list: Vec<usize> = (1..=string.len()).collect();
		string.chars().zip(index_list.iter())
						.filter(|(c, _)| *c == '\n')
						.fold(acc, |mut list, (c, idx)| {list.push(*idx); list})
	}

	pub fn new() -> Buffer {
		let first_node = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
		Buffer {
			original_str: String::new(),
			added_str: String::new(),
			node_list: vec![first_node],
		}
	}

	pub fn with_contents(original_str: String) -> Buffer {
		let offsets = Buffer::get_offsets(&original_str);
		let first_node = BufferNode::new(BufferType::Original, 0, original_str.len(), offsets);
		Buffer {
			original_str: String::new(),
			added_str: String::new(),
			node_list: vec![first_node],
		}
	}

	pub fn insert(&mut self, cursor: &mut Cursor, ch: char) {
		todo!();
	}

	pub fn insert_str(&mut self, cursor: &mut Cursor, string: &str) {
		todo()!;
	}

	pub fn remove(&mut self, cursor: &mut Cursor) -> char {
		todo!();
	}

	pub fn remove_word(&mut self, cursor: &mut Cursor) -> char {
		todo!();
	}

	pub fn as_str(&self) -> &str {
		todo!();
	}

	pub fn len(&self) -> usize {
		todo!();
	}


	pub fn move_cursor_up(&self, cursor: &mut Cursor) {
		todo!();
	}

	pub fn move_cursor_down(&self, cursor: &mut Cursor) {
		todo!();
	}

	pub fn move_cursor_left(&self, cursor: &mut Cursor) {
		todo!();
	}

	pub fn move_cursor_right(&self, cursor: &mut Cursor) {
		todo!();
	}

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_offsets_none() {
		let string = String::from("abc");
		let offsets = Buffer::get_offsets(&string);
		assert_eq!(offsets, &[0]);
	}

	#[test]
	fn test_get_offsets_first() {
		let string = String::from("\nabc");
		let offsets = Buffer::get_offsets(&string);
		assert_eq!(offsets, &[0, 1]);
	}

	#[test]
	fn test_get_offsets_last() {
		let string = String::from("abc\n");
		let offsets = Buffer::get_offsets(&string);
		assert_eq!(offsets, &[0, 4]);
	}

	#[test]
	fn test_get_offsets_only() {
		let string = String::from("\n");
		let offsets = Buffer::get_offsets(&string);
		assert_eq!(offsets, &[0, 1]);
	}

	#[test]
	fn test_get_offsets_two() {
		let string = String::from("\nabc\n");
		let offsets = Buffer::get_offsets(&string);
		assert_eq!(offsets, &[0, 1, 5]);
	}

	#[test]
	fn test_get_offsets_consecutive() {
		let string = String::from("\n\n\n");
		let offsets = Buffer::get_offsets(&string);
		assert_eq!(offsets, &[0, 1, 2, 3]);
	}

}
