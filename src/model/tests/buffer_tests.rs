#[cfg(test)]
mod buffer_tests {
    use super::super::*;

    fn stov(string: &str) -> Vec<u8> {
        let string = String::from(string);
        string.into_bytes()
    }

    #[test]
    fn get_offsets_on_no_newlines() {
        assert_eq!(Buffer::get_offsets("abced "), [0]);
    }

    #[test]
    fn get_offsets_on() {
        assert_eq!(Buffer::get_offsets("\n"), [0, 1]);
    }

    #[test]
    fn get_offsets_on_mixed() {
        assert_eq!(Buffer::get_offsets("\na\n \n"), [0, 1, 3, 5]);
    }

    #[test]
    fn insert_on_empty() {
        let mut buffer = Buffer::new();
        buffer.insert('a');

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 1);
        assert_eq!(cursor.line_idx, 0);
        assert_eq!(cursor.line_offset, 1);
        assert_eq!(cursor.original_line_offset, cursor.line_offset);

        assert_eq!(buffer.original_str, stov(""));
        assert_eq!(buffer.added_str, stov("a"));

        let node_0 = BufferNode::new(BufferType::Added, 0, 1, vec![0]);
        assert_eq!(buffer.node_list, vec![node_0]);
        assert_eq!(buffer.node_list.index(), 0);

        assert_eq!(buffer.current_line, 0);
    }

    #[test]
    fn insert_cursor_before_node() {
        let mut buffer = Buffer::with_contents(String::from("a"));
        buffer.insert('b');

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("a"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("b"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Added, 0, 1, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 0, 1, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 1, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn insert_cursor_after_node() {
        let mut buffer = Buffer::with_contents(String::from("a"));
        buffer.cursor.node_offset = 1;
        buffer.cursor.line_offset = 1;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;
        buffer.insert('b');

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 1, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("a"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("b"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 1, vec![0]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 1, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 1, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn insert_cursor_mid_node() {
        let mut buffer = Buffer::with_contents(String::from("ac"));
        buffer.cursor.node_offset = 1;
        buffer.cursor.line_offset = 1;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;
        buffer.insert('b');

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("ac"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("b"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 1, vec![0]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 1, vec![0]);
        let node_2 = BufferNode::new(BufferType::Original, 1, 1, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 2, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn insert_into_empty_with_newline() {
        let mut buffer = Buffer::new();
        buffer.insert_str(String::from("1\n3"));

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 3, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_idx, 1, "cursor.line_idx mismatch!");
        assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov(""), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("1\n3"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 2]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn insert_cursor_after_node_with_newline_in_added() {
        let mut buffer = Buffer::with_contents(String::from("12"));
        buffer.cursor.node_offset = 2;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;
        buffer.insert_str(String::from("3\n4"));

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 3, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_idx, 1, "cursor.line_idx mismatch!");
        assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("12"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("3\n4"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 2, vec![0]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 2]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 1, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn insert_cursor_after_node_with_newline_in_original() {
        let mut buffer = Buffer::with_contents(String::from("1\n2"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 1;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;
        buffer.insert_str(String::from("34"));

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(cursor.line_offset, 3, "cursor.line_offset mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("1\n2"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("34"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 2]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 2, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 1, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn insert_cursor_after_node_with_newline_in_both() {
        let mut buffer = Buffer::with_contents(String::from("1\n2"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 1;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;
        buffer.insert_str(String::from("3\n4"));

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 3, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_idx, 1, "cursor.line_idx mismatch!");
        assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("1\n2"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("3\n4"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 2]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 2]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 1, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 2, "buffer.current_line mismatch!");
    }

    #[test]
    fn remove_on_empty_buffer() {
        let mut buffer = Buffer::new();
        buffer.remove();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov(""), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        assert_eq!(buffer.node_list, vec![], "node_list contents mismatch!");
        assert!(buffer.node_list.is_empty(), "node_list is not empty!");

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn remove_before_node() {
        let mut buffer = Buffer::with_contents(String::from("abc"));
        buffer.remove();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 0);

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn remove_first_of_node() {
        let mut buffer = Buffer::with_contents(String::from("abc"));
        buffer.current_line = 0;
        buffer.cursor.node_offset = 1;
        buffer.cursor.line_offset = 1;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.remove();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 1, 2, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 0);

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn remove_end_of_node() {
        let mut buffer = Buffer::with_contents(String::from("abc"));
        buffer.current_line = 0;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 3;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.remove();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 2, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 0);

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn remove_mid_of_node() {
        let mut buffer = Buffer::with_contents(String::from("abc"));
        buffer.current_line = 0;
        buffer.cursor.node_offset = 2;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.remove();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 1, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 2, 1, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 1);

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn remove_end_of_node_before_another() {
        let mut buffer = Buffer::with_contents(String::from("abc"));
        buffer.current_line = 0;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 3;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.insert_str(String::from("def"));
        buffer.current_line = 0;
        buffer.cursor.node_offset = 0;
        buffer.cursor.line_offset = 3;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.remove();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("def"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 2, vec![0]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 1);

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn remove_newline_end_of_node() {
        let mut buffer = Buffer::with_contents(String::from("ab\n"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 0;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.remove();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("ab\n"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 2, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 0);

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn remove_newline_end_of_node_with_other_newline() {
        let mut buffer = Buffer::with_contents(String::from("\nb\n"));
        buffer.current_line = 2;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 0;
        buffer.cursor.line_idx = 2;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.remove();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 1, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("\nb\n"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 2, vec![0, 1]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 0);

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn remove_newline_from_other_node_at_mid() {
        let mut buffer = Buffer::with_contents(String::from("\nbc"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.insert_str(String::from("def"));
        buffer.insert_str(String::from("g\ni"));

        buffer.current_line = 2;
        buffer.cursor.node_offset = 2;
        buffer.cursor.line_offset = 0;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.remove();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 6, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("\nbc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("defg\ni"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 1]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 1, vec![0]);
        let node_3 = BufferNode::new(BufferType::Added, 5, 1, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2, node_3],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 3, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn remove_newline_from_other_node_at_end() {
        let mut buffer = Buffer::with_contents(String::from("\nbc"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.insert_str(String::from("def"));
        buffer.insert_str(String::from("gh\n"));

        buffer.current_line = 2;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 0;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.remove();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 7, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("\nbc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("defgh\n"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 1]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 2, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 2, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn remove_newline_from_other_node_at_front() {
        let mut buffer = Buffer::with_contents(String::from("\nbc"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.insert_str(String::from("de\n"));
        buffer.insert_str(String::from("ghi"));

        buffer.current_line = 2;
        buffer.cursor.node_offset = 0;
        buffer.cursor.line_offset = 0;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.remove();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 4, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("\nbc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("de\nghi"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 1]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 2, vec![0]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 3, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 2, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn as_str_on_empty_buffer() {
        let mut buffer = Buffer::new();
        let string = buffer.as_str();
        assert_eq!(string, "");
    }

    #[test]
    fn as_str_on_non_empty_buffer() {
        let mut buffer = Buffer::with_contents(String::from("\nbc"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.insert_str(String::from("de\n"));
        buffer.insert_str(String::from("ghi"));

        buffer.node_list.move_left();

        let string = buffer.as_str();
        assert_eq!(string, "\nbcde\nghi");
    }

    #[test]
    fn as_str_split_by_cursors_on_empty_buffer() {
        let buffer = Buffer::new();
        assert_eq!(buffer.as_str_split_by_cursors(), vec![String::from("")]);
    }

    #[test]
    fn as_str_split_by_cursors_on_non_empty_buffer() {
        let mut buffer = Buffer::with_contents(String::from("\nbc"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.insert_str(String::from("de\n"));
        buffer.insert_str(String::from("ghi"));

        buffer.node_list.move_left();
        buffer.current_line = 1;
        buffer.cursor.node_offset = 1;
        buffer.cursor.line_offset = 4;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;
        let string = buffer.as_str_split_by_cursors();

        assert_eq!(string, vec![String::from("\nbcd"), String::from("e\nghi")]);
    }

    #[test]
    fn move_cursor_left_on_empty_buffer() {
        let mut buffer = Buffer::new();
        buffer.move_cursor_left();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov(""), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        assert_eq!(buffer.node_list, vec![], "node_list contents mismatch!");
        assert!(buffer.node_list.is_empty(), "node_list is not empty!");
        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_left_idempotent_on_node() {
        let mut buffer = Buffer::with_contents(String::from("abc"));
        buffer.current_line = 0;
        buffer.cursor.node_offset = 0;
        buffer.cursor.line_offset = 0;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.move_cursor_left();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_left_on_node() {
        let mut buffer = Buffer::with_contents(String::from("abc"));
        buffer.current_line = 0;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 3;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.move_cursor_left();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_left_on_node_over_newline() {
        let mut buffer = Buffer::with_contents(String::from("ab\n"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 0;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.move_cursor_left();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("ab\n"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 3]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_left_on_node_from_start_of_next() {
        let mut buffer = Buffer::with_contents(String::from("\nbc"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.insert_str(String::from("\nef"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 0;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.move_cursor_left();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 1, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("\nbc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("\nef"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 1]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 1]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_left_over_newline_from_other_node_at_front() {
        let mut buffer = Buffer::with_contents(String::from("\nbc"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.insert_str(String::from("de\n"));
        buffer.insert_str(String::from("ghi"));

        buffer.current_line = 2;
        buffer.cursor.node_offset = 0;
        buffer.cursor.line_offset = 0;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.move_cursor_left();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 4, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("\nbc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("de\nghi"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 1]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 3]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 3, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 1, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_left_over_newline_from_other_node_at_mid() {
        let mut buffer = Buffer::with_contents(String::from("\nbc"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.insert_str(String::from("def"));
        buffer.insert_str(String::from("g\ni"));

        buffer.current_line = 2;
        buffer.cursor.node_offset = 2;
        buffer.cursor.line_offset = 0;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.move_cursor_left();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 1, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 6, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("\nbc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("defg\ni"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 1]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 3, vec![0, 2]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 2, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_left_over_newline_from_other_node_at_end() {
        let mut buffer = Buffer::with_contents(String::from("\nbc"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.insert_str(String::from("def"));
        buffer.insert_str(String::from("gh\n"));

        buffer.current_line = 2;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 0;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.move_cursor_left();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 7, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("\nbc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("defgh\n"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 1]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 3, vec![0, 3]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 2, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_right_on_empty_buffer() {
        let mut buffer = Buffer::new();
        buffer.move_cursor_right();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov(""), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        assert_eq!(buffer.node_list, vec![], "node_list contents mismatch!");
        assert!(buffer.node_list.is_empty(), "node_list is not empty!");
        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_right_idempotent_on_node() {
        let mut buffer = Buffer::with_contents(String::from("abc"));
        buffer.current_line = 0;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 3;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.move_cursor_right();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 3, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 3, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_right_on_node() {
        let mut buffer = Buffer::with_contents(String::from("abc"));
        buffer.current_line = 0;
        buffer.cursor.node_offset = 0;
        buffer.cursor.line_offset = 0;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.move_cursor_right();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 1, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_right_on_node_over_newline() {
        let mut buffer = Buffer::with_contents(String::from("ab\n"));
        buffer.current_line = 0;
        buffer.cursor.node_offset = 2;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 0;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.move_cursor_right();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 3, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 1, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("ab\n"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 3]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_right_on_node_to_start_of_next() {
        let mut buffer = Buffer::with_contents(String::from("\nbc"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.insert_str(String::from("\nef"));

        buffer.node_list.move_left();
        buffer.current_line = 1;
        buffer.cursor.node_offset = 2;
        buffer.cursor.line_offset = 1;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.move_cursor_right();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("\nbc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("\nef"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 1]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 1]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 1, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_right_on_node_to_start_of_next_over_newline() {
        let mut buffer = Buffer::with_contents(String::from("\nbc"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 2;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.insert_str(String::from("\nef"));

        buffer.node_list.move_left();
        buffer.current_line = 1;
        buffer.cursor.node_offset = 2;
        buffer.cursor.line_offset = 1;
        buffer.cursor.line_idx = 1;
        buffer.cursor.original_line_offset = buffer.cursor.line_offset;

        buffer.move_cursor_right();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("\nbc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("\nef"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 1]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 1]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 1, "node_list index mismatch!");

        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_on_empty_buffer() {
        let mut buffer = Buffer::new();
        buffer.move_cursor_up();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov(""), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");
        assert_eq!(buffer.node_list, vec![], "node_list contents mismatch!");
        assert!(buffer.node_list.is_empty(), "node_list is not empty!");
        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_with_newline_in_the_same_node() {
        let mut buffer = Buffer::with_contents(String::from("\nbc\nefg"));
        buffer.current_line = 2;
        buffer.cursor.node_offset = 7;
        buffer.cursor.line_offset = 3;
        buffer.cursor.original_line_offset = 3;
        buffer.cursor.line_idx = 2;

        buffer.move_cursor_up();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 3, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 1, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, 3,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(
            buffer.original_str,
            stov("\nbc\nefg"),
            "original_str mismatch!"
        );
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 7, vec![0, 1, 4]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_with_newline_in_the_previous_node_index_1() {
        let mut buffer = Buffer::with_contents(String::from("\nbc\nefg"));
        buffer.current_line = 2;
        buffer.cursor.node_offset = 7;
        buffer.cursor.line_offset = 3;
        buffer.cursor.original_line_offset = 3;
        buffer.cursor.line_idx = 2;

        buffer.insert_str(String::from("hij"));
        buffer.insert_str(String::from("k\nmno"));

        buffer.current_line = 3;
        buffer.cursor.node_offset = 4;
        buffer.cursor.line_offset = 2;
        buffer.cursor.original_line_offset = 2;
        buffer.cursor.line_idx = 1;

        buffer.move_cursor_up();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 6, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 2, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, 2,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(
            buffer.original_str,
            stov("\nbc\nefg"),
            "original_str mismatch!"
        );
        assert_eq!(buffer.added_str, stov("hijk\nmno"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 7, vec![0, 1, 4]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 5, vec![0, 2]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 2, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_with_newline_in_the_previous_node_index_1_into_next_node() {
        let mut buffer = Buffer::with_contents(String::from("\nbc\nefg"));
        buffer.current_line = 2;
        buffer.cursor.node_offset = 7;
        buffer.cursor.line_offset = 3;
        buffer.cursor.original_line_offset = 3;
        buffer.cursor.line_idx = 2;

        buffer.insert_str(String::from("hij"));
        buffer.insert_str(String::from("k\nmno"));

        buffer.current_line = 3;
        buffer.cursor.node_offset = 5;
        buffer.cursor.line_offset = 3;
        buffer.cursor.original_line_offset = 3;
        buffer.cursor.line_idx = 1;

        buffer.move_cursor_up();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 3, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, 3,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(
            buffer.original_str,
            stov("\nbc\nefg"),
            "original_str mismatch!"
        );
        assert_eq!(buffer.added_str, stov("hijk\nmno"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 7, vec![0, 1, 4]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 5, vec![0, 2]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 1, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 2, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_with_newline_in_the_previous_node_index_1_into_original_node() {
        let mut buffer = Buffer::with_contents(String::from("\nbc\nefg"));
        buffer.current_line = 2;
        buffer.cursor.node_offset = 7;
        buffer.cursor.line_offset = 3;
        buffer.cursor.original_line_offset = 3;
        buffer.cursor.line_idx = 2;

        buffer.insert_str(String::from("hij"));
        buffer.insert_str(String::from("k\nmnopqrstuv"));
        assert_eq!(buffer.cursor.original_line_offset, 10);
        buffer.move_cursor_up();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 1, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 7, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, 10,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(
            buffer.original_str,
            stov("\nbc\nefg"),
            "original_str mismatch!"
        );
        assert_eq!(
            buffer.added_str,
            stov("hijk\nmnopqrstuv"),
            "added_str mismatch!"
        );

        let node_0 = BufferNode::new(BufferType::Original, 0, 7, vec![0, 1, 4]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 12, vec![0, 2]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 2, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 2, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_with_newline_in_the_previous_node_index_0_into_original_node() {
        let mut buffer = Buffer::with_contents(String::from("\nbc\nefg"));
        buffer.current_line = 2;
        buffer.cursor.node_offset = 7;
        buffer.cursor.line_offset = 3;
        buffer.cursor.original_line_offset = 3;
        buffer.cursor.line_idx = 2;

        buffer.insert_str(String::from("hij"));
        buffer.insert_str(String::from("k\nmnopqrstuv"));
        assert_eq!(buffer.cursor.original_line_offset, 10);
        buffer.current_line = 2;
        buffer.cursor.node_offset = 1;
        buffer.cursor.line_offset = 7;
        buffer.cursor.original_line_offset = 7;
        buffer.cursor.line_idx = 0;

        buffer.move_cursor_up();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 3, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 1, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, 7,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(
            buffer.original_str,
            stov("\nbc\nefg"),
            "original_str mismatch!"
        );
        assert_eq!(
            buffer.added_str,
            stov("hijk\nmnopqrstuv"),
            "added_str mismatch!"
        );

        let node_0 = BufferNode::new(BufferType::Original, 0, 7, vec![0, 1, 4]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 12, vec![0, 2]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_with_newline_in_the_previous_node_index_0_into_middle_node() {
        let mut buffer = Buffer::with_contents(String::from("\nbcde\ng"));
        buffer.current_line = 2;
        buffer.cursor.node_offset = 7;
        buffer.cursor.line_offset = 3;
        buffer.cursor.original_line_offset = 3;
        buffer.cursor.line_idx = 2;

        buffer.insert_str(String::from("h\nj"));
        buffer.insert_str(String::from("kl\nnopqrstuv"));
        buffer.current_line = 3;
        buffer.cursor.node_offset = 2;
        buffer.cursor.line_offset = 3;
        buffer.cursor.original_line_offset = 3;
        buffer.cursor.line_idx = 0;

        buffer.move_cursor_up();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 1, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 2, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, 3,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(
            buffer.original_str,
            stov("\nbcde\ng"),
            "original_str mismatch!"
        );
        assert_eq!(
            buffer.added_str,
            stov("h\njkl\nnopqrstuv"),
            "added_str mismatch!"
        );

        let node_0 = BufferNode::new(BufferType::Original, 0, 7, vec![0, 1, 6]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 2]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 12, vec![0, 3]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 1, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 2, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_with_newline_in_the_previous_node_index_0_into_start_of_middle_node() {
        let mut buffer = Buffer::with_contents(String::from("\nbcde\ng"));
        buffer.current_line = 2;
        buffer.cursor.node_offset = 7;
        buffer.cursor.line_offset = 3;
        buffer.cursor.original_line_offset = 3;
        buffer.cursor.line_idx = 2;

        buffer.insert_str(String::from("h\nj"));
        buffer.insert_str(String::from("kl\nnopqrstuv"));
        buffer.current_line = 3;
        buffer.cursor.node_offset = 0;
        buffer.cursor.line_offset = 1;
        buffer.cursor.original_line_offset = 1;
        buffer.cursor.line_idx = 0;

        buffer.move_cursor_up();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, 1,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(
            buffer.original_str,
            stov("\nbcde\ng"),
            "original_str mismatch!"
        );
        assert_eq!(
            buffer.added_str,
            stov("h\njkl\nnopqrstuv"),
            "added_str mismatch!"
        );

        let node_0 = BufferNode::new(BufferType::Original, 0, 7, vec![0, 1, 6]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0, 2]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 12, vec![0, 3]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 1, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 2, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_down_on_empty_buffer() {
        let mut buffer = Buffer::new();
        buffer.move_cursor_down();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov(""), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");
        assert_eq!(buffer.node_list, vec![], "node_list contents mismatch!");
        assert!(buffer.node_list.is_empty(), "node_list is not empty!");
        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_on_same_node_without_newline() {
        let mut buffer = Buffer::with_contents(String::from("abc"));
        buffer.move_cursor_down();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("abc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_on_same_node_with_newline() {
        let mut buffer = Buffer::with_contents(String::from("a\nc"));
        buffer.current_line = 0;
        buffer.cursor.node_offset = 0;
        buffer.cursor.line_offset = 0;
        buffer.cursor.original_line_offset = 0;
        buffer.cursor.line_idx = 0;

        buffer.move_cursor_down();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 1, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, 0,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("a\nc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 2]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_on_same_node_with_newline_2() {
        let mut buffer = Buffer::with_contents(String::from("a\nc"));
        buffer.current_line = 0;
        buffer.cursor.node_offset = 1;
        buffer.cursor.line_offset = 1;
        buffer.cursor.original_line_offset = 1;
        buffer.cursor.line_idx = 0;

        buffer.move_cursor_down();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 3, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 1, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, 1,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("a\nc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 2]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_to_next_node_with_two_newlines() {
        let mut buffer = Buffer::with_contents(String::from("a\n\n"));
        buffer.current_line = 0;
        buffer.cursor.node_offset = 1;
        buffer.cursor.line_offset = 1;
        buffer.cursor.original_line_offset = 1;
        buffer.cursor.line_idx = 0;

        buffer.move_cursor_down();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 2, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 0, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 1, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, 1,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("a\n\n"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov(""), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 2, 3]);
        assert_eq!(
            buffer.node_list,
            vec![node_0],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 0, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_to_between_two_nodes() {
        let mut buffer = Buffer::with_contents(String::from("a\nc"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 3;
        buffer.cursor.line_offset = 1;
        buffer.cursor.original_line_offset = 1;
        buffer.cursor.line_idx = 1;
        buffer.insert_str(String::from("def"));
        buffer.node_list.move_left();
        buffer.current_line = 0;
        buffer.cursor.node_offset = 1;
        buffer.cursor.line_offset = 1;
        buffer.cursor.original_line_offset = 1;
        buffer.cursor.line_idx = 0;

        buffer.move_cursor_down();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 1, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, 1,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov("a\nc"), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("def"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 3, vec![0, 2]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 1, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 1, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_to_across_another_nodes_to_end_of_tail() {
        let mut buffer = Buffer::with_contents(String::from("\nbcdef\nhi"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 9;
        buffer.cursor.line_offset = 2;
        buffer.cursor.original_line_offset = 2;
        buffer.cursor.line_idx = 2;
        buffer.insert_str(String::from("jkl"));
        buffer.insert_str(String::from("mno"));
        buffer.insert_str(String::from("pqr"));
        buffer.node_list.move_left();
        buffer.node_list.move_left();
        buffer.node_list.move_left();
        buffer.current_line = 1;
        buffer.cursor.node_offset = 6;
        buffer.cursor.line_offset = 5;
        buffer.cursor.original_line_offset = 20;
        buffer.cursor.line_idx = 1;

        buffer.move_cursor_down();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 3, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 11, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, 20,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(
            buffer.original_str,
            stov("\nbcdef\nhi"),
            "original_str mismatch!"
        );
        assert_eq!(buffer.added_str, stov("jklmnopqr"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Original, 0, 9, vec![0, 1, 7]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 3, vec![0]);
        let node_3 = BufferNode::new(BufferType::Added, 6, 3, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2, node_3],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 3, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 2, "buffer.current_line mismatch!");
    }

    #[test]
    fn move_cursor_up_from_last_line_of_node_across_multiple() {
        let mut buffer = Buffer::with_contents(String::from("\nbcdeghi"));
        buffer.current_line = 1;
        buffer.cursor.node_offset = 8;
        buffer.cursor.line_offset = 7;
        buffer.cursor.original_line_offset = 7;
        buffer.cursor.line_idx = 1;
        buffer.insert_str(String::from("jkl"));
        buffer.insert_str(String::from("\nno"));
        buffer.insert_str(String::from("pqrs"));
        buffer.insert_str(String::from("tuv"));
        buffer.node_list.move_left();
        buffer.node_list.move_left();
        buffer.node_list.move_left();
        buffer.node_list.move_left();
        buffer.current_line = 1;
        buffer.cursor.node_offset = 6;
        buffer.cursor.line_offset = 5;
        buffer.cursor.original_line_offset = 5;
        buffer.cursor.line_idx = 1;

        buffer.move_cursor_down();

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 3, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_offset, 5, "cursor.line_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(
            cursor.original_line_offset, 5,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(
            buffer.original_str,
            stov("\nbcdeghi"),
            "original_str mismatch!"
        );
        assert_eq!(
            buffer.added_str,
            stov("jkl\nnopqrstuv"),
            "added_str mismatch!"
        );

        let node_0 = BufferNode::new(BufferType::Original, 0, 8, vec![0, 1]);
        let node_1 = BufferNode::new(BufferType::Added, 0, 3, vec![0]);
        let node_2 = BufferNode::new(BufferType::Added, 3, 3, vec![0, 1]);
        let node_3 = BufferNode::new(BufferType::Added, 6, 4, vec![0]);
        let node_4 = BufferNode::new(BufferType::Added, 10, 3, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2, node_3, node_4],
            "node_list contents mismatch!"
        );

        assert_eq!(buffer.node_list.index(), 3, "node_list index mismatch!");
        assert_eq!(buffer.current_line, 2, "buffer.current_line mismatch!");
    }

    #[test]
    fn random_test() {
        let mut buffer = Buffer::new();

        buffer.insert('1');
        buffer.insert('2');
        buffer.insert('3');
        buffer.insert('4');

        println!("{:?}", buffer.cursor);
        println!("{:?}", buffer.node_list.index());
        buffer.move_cursor_left();
        println!("{:?}", buffer.cursor);
        println!("{:?}", buffer.node_list.index());
        buffer.insert('5');
        buffer.insert('6');

        let cursor = buffer.cursor;
        assert_eq!(cursor.node_offset, 0, "cursor.node_offset mismatch!");
        assert_eq!(cursor.line_idx, 0, "cursor.line_idx mismatch!");
        assert_eq!(cursor.line_offset, 5, "cursor.line_offset mismatch!");
        assert_eq!(
            cursor.original_line_offset, cursor.line_offset,
            "cursor.line_offset mismatch!"
        );

        assert_eq!(buffer.original_str, stov(""), "original_str mismatch!");
        assert_eq!(buffer.added_str, stov("123456"), "added_str mismatch!");

        let node_0 = BufferNode::new(BufferType::Added, 0, 1, vec![0]);
        let node_1 = BufferNode::new(BufferType::Added, 1, 1, vec![0]);
        let node_2 = BufferNode::new(BufferType::Added, 2, 1, vec![0]);
        let node_3 = BufferNode::new(BufferType::Added, 4, 1, vec![0]);
        let node_4 = BufferNode::new(BufferType::Added, 5, 1, vec![0]);
        let node_5 = BufferNode::new(BufferType::Added, 3, 1, vec![0]);
        assert_eq!(
            buffer.node_list,
            vec![node_0, node_1, node_2, node_3, node_4, node_5],
            "node_list contents mismatch!"
        );
        assert_eq!(buffer.node_list.index(), 5);

        assert_eq!(buffer.current_line, 0, "buffer.current_line mismatch!");
    }
}
