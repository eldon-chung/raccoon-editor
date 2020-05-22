use crate::utils::Cursor;
#[allow(dead_code)]


#[derive(Copy, Clone, PartialEq, Debug)]
enum BufferType {
	Original,
	Added,
}

#[derive(Clone, PartialEq, Debug)]
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

	fn last_line_offset(&self) -> usize {
		*self.line_offsets.last().unwrap()
	}
}

pub struct Buffer {
	original_str: Vec<char>,
	added_str: Vec<char>,
	node_list: Vec<BufferNode>,

	// an important invariant is how the cursor is going to be placed.
	//	this is going to be maintained throughout the operations.
	//  if the cursor is in between two nodes, it will always be at the 0 index of the 
	//	node on the right, rather than the end index of the left node.
	//	the only time the cursor will be at the end index of a node is if there is
	// 	no other node to its right.
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
			original_str: Vec::new(),
			added_str: Vec::new(),
			node_list: vec![first_node],
		}
	}

	pub fn with_contents(original_str: String) -> Buffer {
		let offsets = Buffer::get_offsets(&original_str);
		let first_node = BufferNode::new(BufferType::Original, 0, original_str.len(), offsets);
		Buffer {
			original_str: original_str.chars().collect(),
			added_str: Vec::new(),
			node_list: vec![first_node],
		}
	}

	pub fn insert(&mut self, cursor: &mut Cursor, ch: char) {
		let mut string = String::new();
		string.push(ch);
		self.insert_str(cursor, string);
	}

	pub fn insert_str(&mut self, cursor: &mut Cursor, string: String) {
		// get the node fields and add the new node
		let line_offsets = Buffer::get_offsets(&string);
		let num_lines = line_offsets.len();
		let idx = self.added_str.len();
		let offset = string.len();
		let new_node = BufferNode::new(BufferType::Added, idx, offset, line_offsets);

		// append string to add_str
		let mut vec_converted = string.chars().collect::<Vec<char>>();
		self.added_str.append(&mut vec_converted);

		// split the current node we are on into two
		let node_to_split = &mut self.node_list[cursor.node_idx];
		if cursor.node_offset == node_to_split.offset() {
			// just insert the new node after the current one
			// this should only happen when the cursor is at the end of the buffer
			assert!(cursor.node_idx == self.node_list.len() - 1);
			// update cursor before losing ownership of the new node
			cursor.node_idx += 1;
			cursor.node_offset = new_node.offset();
			cursor.line_idx = num_lines - 1;
			let last_offset = new_node.last_line_offset();
			cursor.line_offset = cursor.node_offset - last_offset;
			self.node_list.push(new_node);

		} else if cursor.node_offset == 0 {
			// just insert the new node before the current one
			self.node_list.insert(cursor.node_offset, new_node);
			cursor.node_idx += 1;
			assert_eq!(cursor.node_offset, 0);
			assert_eq!(cursor.line_idx, 0);
			assert_eq!(cursor.line_offset, 0);
		} else {
			// split the node into two and insert it in between
			let from = node_to_split.from();
			let index = node_to_split.index();
			let offset = node_to_split.offset();
			let line_offsets = &mut node_to_split.line_offsets();

			// split the line_offsets based on where the cursor is
			let drain_from_idx = cursor.line_idx;
			let left_line_offsets = line_offsets.drain(..=drain_from_idx).collect();
			let node_offset = cursor.node_offset;
			let mut right_line_offsets: Vec<usize> = line_offsets.drain( .. )
												.map(|x| x - node_offset)
												.collect();
			right_line_offsets.insert(0, 0);
			// construct left and right nodes
			let left_node = BufferNode::new(from, index, cursor.node_offset, left_line_offsets );
			let right_node = BufferNode::new(from, index + cursor.node_offset, offset - cursor.node_offset, right_line_offsets);

			// do this in case we're inserting at the end of the vector
			self.node_list.insert(cursor.node_idx, left_node);
			self.node_list.insert(cursor.node_idx + 1, new_node);
			self.node_list.insert(cursor.node_idx + 2, right_node);
			self.node_list.remove(cursor.node_idx + 3);

			// update the cursor
			cursor.node_idx += 2;
			cursor.line_offset = 0;
			cursor.node_offset = 0;
			cursor.line_idx = 0;
		}
	}

	pub fn remove(&mut self, cursor: &mut Cursor) {
		// in general there are two cases, whether the cursor is between
		// 	two nodes or in the middle of a single node. both cases are handled
		//  quite differently

		let mut node_idx = cursor.node_idx;
		let mut node_offset = cursor.node_offset;


		if node_offset == 0 {
			// if we're at the head of the file there's nothing to do;
			if node_idx == 0 { return; }
			// else reference the previous node
			node_idx -= 1;
			node_offset = self.node_list[node_idx].offset();
		}

		let from = self.node_list[node_idx].from();
		let index = self.node_list[node_idx].index();
		let line_offsets = &mut self.node_list[node_idx].line_offsets();
		let mut line_offsets: Vec<usize> = line_offsets.drain(..).collect();

		if node_offset == self.node_list[node_idx].offset() {
			// an edge case is that this is the empty node that appears when
			// we've just constructed a file. in that case we should return as well
			if node_idx == 0 &&  node_offset == 0 { return; }
			// just reduce the offset value by 1 here
			node_offset -= 1;
			if node_offset == 0 {
				// node is now empty, just delete it and move the cursor
				self.node_list.remove(node_idx);
				cursor.node_offset = 0;
				cursor.line_idx = 0;
				cursor.line_offset = 0;
				return;
			}
			let removed_char = match from {
				BufferType::Original => self.original_str[index + node_offset],
				BufferType::Added => self.added_str[index + node_offset],
			};
			if removed_char == '\n' {
				line_offsets.pop();
			}
			let new_node = BufferNode::new(from, index, node_offset, line_offsets);
			// in this case the cursor position does not need to be updated?
			//  unless the cursor was at the end of the entire buffer
			if cursor.node_idx == self.node_list.size() - 1 {
				cursor.node_offset -= 1;
			}
			self.node_list.insert(node_idx, new_node);
			self.node_list.remove(node_idx + 1);
		} else {
			// split the line_offsets based on where the cursor is
			let drain_from_idx = cursor.line_idx;
			let left_line_offsets = line_offsets.drain(..=drain_from_idx).collect();
			let node_offset = cursor.node_offset;
			let mut right_line_offsets: Vec<usize> = line_offsets.drain( .. )
												.map(|x| x - node_offset)
												.collect();
			right_line_offsets.insert(0, 0);

			let removed_char = match from {
				BufferType::Original => self.original_str[index + node_offset],
				BufferType::Added => self.added_str[index + node_offset],
			};
			if removed_char == '\n' {
				left_line_offsets.pop();
			}

			// construct the left and right node
			let left_node = BufferNode::new(from, index, cursor.node_offset - 1, left_line_offsets);
			let right_node = BufferNode::new(from, index + cursor.node_offset, offset - cursor.node_offset, right_line_offsets);

			if cursor.node_offset - 1 == 0 {
				// the left node is empty, we just need to insert the right node
				self.node_list.insert(node_idx, right_node);
				self.node_list.remove(node_idx + 1);

				cursor.node_offset = 0;
				cursor.line_idx = 0;
				cursor.line_offset = 0;
			} else {
				self.node_list.insert(node_idx, left_node);
				self.node_list.insert(node_idx + 1, right_node);
				self.node_list.remove(node_idx + 2);

				cursor.node_idx += 1;
				cursor.node_offset = 0;
				cursor.line_idx = 0;
				cursor.line_offset = 0;
			}
		}

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
	fn get_offsets_none() {
		let string = String::from("abc");
		let offsets = Buffer::get_offsets(&string);
		assert_eq!(offsets, &[0]);
	}

	#[test]
	fn get_offsets_first() {
		let string = String::from("\nabc");
		let offsets = Buffer::get_offsets(&string);
		assert_eq!(offsets, &[0, 1]);
	}

	#[test]
	fn get_offsets_last() {
		let string = String::from("abc\n");
		let offsets = Buffer::get_offsets(&string);
		assert_eq!(offsets, &[0, 4]);
	}

	#[test]
	fn get_offsets_only() {
		let string = String::from("\n");
		let offsets = Buffer::get_offsets(&string);
		assert_eq!(offsets, &[0, 1]);
	}

	#[test]
	fn get_offsets_two() {
		let string = String::from("\nabc\n");
		let offsets = Buffer::get_offsets(&string);
		assert_eq!(offsets, &[0, 1, 5]);
	}

	#[test]
	fn get_offsets_consecutive() {
		let string = String::from("\n\n\n");
		let offsets = Buffer::get_offsets(&string);
		assert_eq!(offsets, &[0, 1, 2, 3]);
	}

	#[test]
	fn insert_char_into_fresh_buffer() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::new();
		buffer.insert(&mut cursor, 'a');

		assert_eq!(buffer.original_str, "", "original_str mismatch");
		assert_eq!(buffer.added_str, "a", "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0] );
		let node_1 = BufferNode::new(BufferType::Added, 0, 1, vec![0] );
		assert_eq!(buffer.node_list, vec![node_0, node_1], "node_list mismatch");

		assert_eq!(cursor.node_idx, 1, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 1, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch");
	}

	#[test]
	fn insert_newl_into_fresh_buffer() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::new();
		buffer.insert(&mut cursor, '\n');

		assert_eq!(buffer.original_str, "", "original_str mismatch");
		assert_eq!(buffer.added_str, "\n", "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0] );
		let node_1 = BufferNode::new(BufferType::Added, 0, 1, vec![0, 1] );
		assert_eq!(buffer.node_list, vec![node_0, node_1], "node_list mismatch");

		assert_eq!(cursor.node_idx, 1, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 1, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 1, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}

	#[test]
	fn insert_newlnewl_into_fresh_buffer() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::new();
		let inserted_str = String::from("\n\n");
		buffer.insert_str(&mut cursor, inserted_str);

		assert_eq!(buffer.original_str, "", "original_str mismatch");
		assert_eq!(buffer.added_str, "\n\n", "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0] );
		let node_1 = BufferNode::new(BufferType::Added, 0, 2, vec![0, 1, 2] );
		assert_eq!(buffer.node_list, vec![node_0, node_1], "node_list mismatch");

		assert_eq!(cursor.node_idx, 1, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 2, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}

	#[test]
	fn insert_before_node() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::with_contents(String::from("abdc\nef"));

		buffer.insert_str(&mut cursor, String::from("1\n2"));

		assert_eq!(buffer.original_str, "abdc\nef", "original_str mismatch");
		assert_eq!(buffer.added_str, "1\n2", "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 2] );
		let node_1 = BufferNode::new(BufferType::Original, 0, 7, vec![0, 5] );
		assert_eq!(buffer.node_list, vec![node_0, node_1], "node_list mismatch");

		assert_eq!(cursor.node_idx, 1, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}

	#[test]
	fn insert_after_node() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::with_contents(String::from("abdc\nef"));
		cursor.node_idx = 0;
		cursor.node_offset = 7;
		cursor.line_idx = 1;
		cursor.line_offset = 2;

		buffer.insert_str(&mut cursor, String::from("1\n2"));

		assert_eq!(buffer.original_str, "abdc\nef", "original_str mismatch");
		assert_eq!(buffer.added_str, "1\n2", "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Original, 0, 7, vec![0, 5] );
		let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 2] );
		assert_eq!(buffer.node_list, vec![node_0, node_1], "node_list mismatch");

		assert_eq!(cursor.node_idx, 1, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 3, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 1, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch");
	}

	#[test]
	fn insert_mid_node() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::with_contents(String::from("abdc\nef\ng"));
		cursor.node_idx = 0;
		cursor.node_offset = 5;
		cursor.line_idx = 1;
		cursor.line_offset = 0;

		buffer.insert_str(&mut cursor, String::from("1\n2"));
		//|abdc\n|1\n2|ef\ng

		assert_eq!(buffer.original_str, "abdc\nef\ng", "original_str mismatch");
		assert_eq!(buffer.added_str, "1\n2", "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Original, 0, 5, vec![0, 5] );
		let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 2] );
		let node_2 = BufferNode::new(BufferType::Original, 5, 4, vec![0, 3] );
		assert_eq!(buffer.node_list, vec![node_0, node_1, node_2], "node_list mismatch");

		assert_eq!(cursor.node_idx, 2, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}
}
