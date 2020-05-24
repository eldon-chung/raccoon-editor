#[cfg(test)]
mod buffer_tests {
	use super::super::*;

	fn stov(string: &'static str) -> Vec<u8> {
		string.as_bytes().to_vec()
	}

	// get_offsets_tests
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

	// insert_tests
	#[test]
	fn insert_char_into_fresh_buffer() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::new();
		buffer.insert(&mut cursor, 'a');

		assert_eq!(buffer.original_str, stov(""), "original_str mismatch");
		assert_eq!(buffer.added_str, stov("a"), "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Added, 0, 1, vec![0] );
		assert_eq!(buffer.node_list, vec![node_0], "node_list mismatch");

		assert_eq!(cursor.node_idx, 0, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 1, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch");
	}

	#[test]
	fn insert_newl_into_fresh_buffer() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::new();
		buffer.insert(&mut cursor, '\n');

		assert_eq!(buffer.original_str, stov(""), "original_str mismatch");
		assert_eq!(buffer.added_str, stov("\n"), "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Added, 0, 1, vec![0, 1] );
		assert_eq!(buffer.node_list, vec![node_0], "node_list mismatch");

		assert_eq!(cursor.node_idx, 0, "cursor.node_idx mismatch");
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

		assert_eq!(buffer.original_str, stov(""), "original_str mismatch");
		assert_eq!(buffer.added_str, stov("\n\n"), "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Added, 0, 2, vec![0, 1, 2] );
		assert_eq!(buffer.node_list, vec![node_0], "node_list mismatch");

		assert_eq!(cursor.node_idx, 0, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 2, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}

	#[test]
	fn insert_before_node() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::with_contents(String::from("abdc\nef"));

		buffer.insert_str(&mut cursor, String::from("1\n2"));

		assert_eq!(buffer.original_str, stov("abdc\nef"), "original_str mismatch");
		assert_eq!(buffer.added_str, stov("1\n2"), "added_str mismatch");

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

		assert_eq!(buffer.original_str, stov("abdc\nef"), "original_str mismatch");
		assert_eq!(buffer.added_str, stov("1\n2"), "added_str mismatch");

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

		assert_eq!(buffer.original_str, stov("abdc\nef\ng"), "original_str mismatch");
		assert_eq!(buffer.added_str, stov("1\n2"), "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Original, 0, 5, vec![0, 5] );
		let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 2] );
		let node_2 = BufferNode::new(BufferType::Original, 5, 4, vec![0, 3] );
		assert_eq!(buffer.node_list, vec![node_0, node_1, node_2], "node_list mismatch");

		assert_eq!(cursor.node_idx, 2, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}

	#[test]
	fn remove_from_fresh_buffer() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::new();

		buffer.remove(&mut cursor);

		assert_eq!(buffer.original_str, stov(""), "original_str mismatch");
		assert_eq!(buffer.added_str, stov(""), "added_str mismatch");

		assert_eq!(buffer.node_list, vec![], "node_list mismatch");

		assert_eq!(cursor.node_idx, 0, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}

	#[test]
	fn remove_single_char_from_buffer() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::with_contents(String::from("a"));
		cursor.node_offset = 1;
		cursor.line_offset = 1;

		buffer.remove(&mut cursor);

		assert_eq!(buffer.original_str, stov("a"), "original_str mismatch");
		assert_eq!(buffer.added_str, stov(""), "added_str mismatch");

		assert_eq!(buffer.node_list, vec![], "node_list mismatch");

		assert_eq!(cursor.node_idx, 0, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}

	#[test]
	fn remove_left_from_node() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::with_contents(String::from("abc"));

		buffer.remove(&mut cursor);

		assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch");
		assert_eq!(buffer.added_str, stov(""), "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0]);
		assert_eq!(buffer.node_list, vec![node_0], "node_list mismatch");

		assert_eq!(cursor.node_idx, 0, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}

	#[test]
	fn remove_from_last_of_node() {
		let mut cursor = Cursor::new();
		cursor.node_offset = 3;
		cursor.line_offset = 3;

		let mut buffer = Buffer::with_contents(String::from("abc"));

		buffer.remove(&mut cursor);

		assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch");
		assert_eq!(buffer.added_str, stov(""), "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Original, 0, 2, vec![0]);
		assert_eq!(buffer.node_list, vec![node_0], "node_list mismatch");

		assert_eq!(cursor.node_idx, 0, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch");
	}

	#[test]
	fn remove_from_mid_of_node() {
		let mut cursor = Cursor::new();
		cursor.node_offset = 2;
		cursor.line_offset = 2;
		let mut buffer = Buffer::with_contents(String::from("abc"));

		buffer.remove(&mut cursor);

		assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch");
		assert_eq!(buffer.added_str, stov(""), "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Original, 0, 1, vec![0]);
		let node_1 = BufferNode::new(BufferType::Original, 2, 1, vec![0]);
		assert_eq!(buffer.node_list, vec![node_0, node_1], "node_list mismatch");

		assert_eq!(cursor.node_idx, 1, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}

	#[test]
	fn remove_twice_from_mid_of_node() {
		let mut cursor = Cursor::new();
		cursor.node_offset = 2;
		let mut buffer = Buffer::with_contents(String::from("abc"));

		buffer.remove(&mut cursor);
		buffer.remove(&mut cursor);

		assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch");
		assert_eq!(buffer.added_str, stov(""), "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Original, 2, 1, vec![0]);
		assert_eq!(buffer.node_list, vec![node_0], "node_list mismatch");

		assert_eq!(cursor.node_idx, 0, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}

	#[test]
	fn remove_with_two_nodes() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::with_contents(String::from("abc"));

		buffer.insert_str(&mut cursor, String::from("\ndef\n"));
		cursor.node_idx = 1;
		cursor.node_offset = 0;
		cursor.line_idx = 0;
		cursor.line_offset = 0;

		buffer.remove(&mut cursor);
		buffer.remove(&mut cursor);

		assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch");
		assert_eq!(buffer.added_str, stov("\ndef\n"), "added_str mismatch");

		let node_0 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 1]);
		let node_1 = BufferNode::new(BufferType::Original, 0, 3, vec![0]);
		assert_eq!(buffer.node_list, vec![node_0, node_1], "node_list mismatch");

		assert_eq!(cursor.node_idx, 1, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}

	#[test]
	fn remove_with_newlines() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::with_contents(String::from("\nde\nf\n"));
		cursor.node_offset = 4;
		cursor.line_idx = 2;
		cursor.line_offset = 0;

		buffer.remove(&mut cursor);

		assert_eq!(buffer.original_str, stov("\nde\nf\n"), "original_str mismatch");

		let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 1]);
		let node_1 = BufferNode::new(BufferType::Original, 4, 2, vec![0, 2]);
		assert_eq!(buffer.node_list, vec![node_0, node_1], "node_list mismatch");

		assert_eq!(cursor.node_idx, 1, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}

	#[test]
	fn insert_4_then_remove_5() {
		let mut cursor = Cursor::new();
		let mut buffer = Buffer::new();

		buffer.insert(&mut cursor, 'a');
		buffer.insert(&mut cursor, 'b');
		buffer.insert(&mut cursor, 'c');
		buffer.insert(&mut cursor, 'd');
		buffer.remove(&mut cursor);
		buffer.remove(&mut cursor);
		buffer.remove(&mut cursor);
		buffer.remove(&mut cursor);
		buffer.remove(&mut cursor);


		assert_eq!(buffer.original_str, stov(""), "original_str mismatch");
		assert_eq!(buffer.added_str, stov("abcd"), "original_str mismatch");

		assert_eq!(buffer.node_list, Vec::new(), "node_list mismatch");

		assert_eq!(cursor.node_idx, 0, "cursor.node_idx mismatch");
		assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch");
		assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch");
		assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch");
	}

	#[test]
	fn get_str_from_fresh_buffer() {
		let mut buffer = Buffer::new();
		let string = buffer.as_str();
		assert_eq!(string, "");
	}

	#[test]
	fn get_str_from_single_node_buffer() {
		let mut buffer = Buffer::with_contents(String::from("\nde\nf\n"));
		let string = buffer.as_str();
		assert_eq!(string, "\nde\nf\n");
	}

	#[test]
	fn get_str_from_multiple_node_buffer() {
		// warning! this test is coupled with the functionality of insert right now
		//	a better way would be to explicitly initialise the contents of the buffer
		let mut buffer = Buffer::with_contents(String::from("\nde\nf\n"));
		let mut cursor = Cursor::new();

		buffer.insert_str(&mut cursor, String::from("ab\nc!\n"));
		cursor.node_idx = 1;
		cursor.node_offset = 6;
		cursor.line_idx = 3;
		cursor.line_offset = 0;
		buffer.insert_str(&mut cursor, String::from("ghi"));

		let string = buffer.as_str();
		assert_eq!(string, "ab\nc!\n\nde\nf\nghi");
	}
}
