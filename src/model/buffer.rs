use std::str;

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
	original_str: Vec<u8>,
	added_str: Vec<u8>,
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
		Buffer {
			original_str: Vec::new(),
			added_str: Vec::new(),
			node_list: Vec::new(),
		}
	}

	pub fn with_contents(original_str: String) -> Buffer {
		let offsets = Buffer::get_offsets(&original_str);
		let first_node = BufferNode::new(BufferType::Original, 0, original_str.len(), offsets);
		Buffer {
			original_str: original_str.as_bytes().to_vec(),
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
		let mut vec_converted = string.as_bytes().to_vec();
		self.added_str.append(&mut vec_converted);

		if self.node_list.len() == 0 {
			cursor.node_idx = 0;
			cursor.node_offset = offset;
			cursor.line_idx = num_lines - 1;
			let last_offset = new_node.last_line_offset();
			cursor.line_offset = cursor.node_offset - last_offset;
			self.node_list.push(new_node);
			return;
		}

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

		if self.node_list.len() == 0 {
			return;
		}

		if cursor.node_idx == 0 && cursor.node_offset == 0 {
			return;
		}

		let mut node_idx = cursor.node_idx;
		let mut node_offset = cursor.node_offset;

		if node_offset == 0 && node_idx >= 1 {
			node_idx -= 1;
			node_offset = self.node_list[node_idx].offset();
		}

		let from = self.node_list[node_idx].from();
		let offset = self.node_list[node_idx].offset();
		let index = self.node_list[node_idx].index();
		let line_offsets = &mut self.node_list[node_idx].line_offsets();
		let mut line_offsets: Vec<usize> = line_offsets.drain(..).collect();

		if node_offset == self.node_list[node_idx].offset() {
			// just reduce the node offset by 1 here
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
			if removed_char == '\n' as u8 {
				line_offsets.pop();
			}
			let new_node = BufferNode::new(from, index, node_offset, line_offsets);
			// in this case the cursor position does not need to be updated?
			//  unless the cursor was at the end of the entire buffer
			if node_idx == self.node_list.len() - 1 {
				cursor.node_offset -= 1;
			}
			self.node_list.insert(node_idx, new_node);
			self.node_list.remove(node_idx + 1);
		} else {
			// split the line_offsets based on where the cursor is
			let drain_from_idx = cursor.line_idx;
			let mut left_line_offsets: Vec<usize> = line_offsets.drain(..=drain_from_idx).collect();
			let node_offset = cursor.node_offset;
			let mut right_line_offsets: Vec<usize> = line_offsets.drain( .. )
												.map(|x| x - node_offset)
												.collect();
			right_line_offsets.insert(0, 0);

			let removed_char = match from {
				BufferType::Original => self.original_str[index + node_offset - 1],
				BufferType::Added => self.added_str[index + node_offset - 1],
			};
			if removed_char == '\n' as u8 {
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

	pub fn as_str(&self) -> String {
		let serialised_str = self.node_list.iter()
								.fold( String::new(), |mut acc, node| {
									let source = match node.from() {
										BufferType::Original => &self.original_str,
										BufferType::Added => &self.added_str,
									};
									let slice = &source[node.index()..node.index() + node.offset()];
									let chunk = str::from_utf8(slice).unwrap();
									acc.push_str(chunk);
									acc
									}
								);
		serialised_str

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
#[path = "tests/buffer_tests.rs"]
mod buffer_tests;