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

    fn reduce_offset_by(&mut self, amount: usize) {
        self.offset -= amount;
    }

    fn from(&self) -> BufferType {
        self.from
    }

    fn line_offsets(&mut self) -> &mut Vec<usize> {
        &mut self.line_offsets
    }

    fn line_offset_at(&self, idx: usize) -> usize {
        self.line_offsets[idx]
    }

    fn line_offsets_len(&self) -> usize {
        self.line_offsets.len()
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
    //  this is going to be maintained throughout the operations.
    //  if the cursor is in between two nodes, it will always be at the 0 index of the 
    //  node on the right, rather than the end index of the left node.
    //  the only time the cursor will be at the end index of a node is if there is
    //  no other node to its right.
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

        // construct the new node to be inserted
        let line_offsets = Buffer::get_offsets(&string);
        let idx = self.added_str.len();
        let offset = string.len();
        let new_node = BufferNode::new(BufferType::Added, idx, offset, line_offsets);

        // append string to add_str
        let mut vec_converted = string.as_bytes().to_vec();
        self.added_str.append(&mut vec_converted);

        if self.node_list.len() == 0 {
            // node_list should be empty, so just push and return
            //  cursor should be referring to the end of the first node
            cursor.node_idx = 0;
            cursor.node_offset = offset;
            cursor.line_idx = new_node.line_offsets_len() - 1;
            cursor.line_offset = cursor.node_offset - new_node.last_line_offset();
            self.node_list.push(new_node);
            return;
        }

        let node_to_split = &mut self.node_list[cursor.node_idx];
        if cursor.node_offset == node_to_split.offset() {
            // cursor should be at the end of the buffer
            assert!(cursor.node_idx == self.node_list.len() - 1);

            // push the new node at the end of node_list
            //  update cursor to point at the end of the new node
            cursor.node_idx += 1;
            cursor.node_offset = new_node.offset();
            cursor.line_idx = new_node.line_offsets_len() - 1;
            cursor.line_offset = cursor.node_offset - new_node.last_line_offset();
            self.node_list.push(new_node);

        } else if cursor.node_offset == 0 {
            // cursor should be at the front of a node
            assert_eq!(cursor.node_offset, 0);
            assert_eq!(cursor.line_idx, 0);
            assert_eq!(cursor.line_offset, 0);
            // just insert the new node before current node
            //	which is where the cursor is
            self.node_list.insert(cursor.node_idx, new_node);
            cursor.node_idx += 1;
        } else {
            // split the node into two and insert it in between
            let from = node_to_split.from();
            let index = node_to_split.index();
            let offset = node_to_split.offset();
            let line_offsets = &mut node_to_split.line_offsets();

            // split the line_offsets based on where the cursor is
            let left_line_offsets = line_offsets.drain(..=cursor.line_idx).collect();
            let mut right_line_offsets: Vec<usize> = line_offsets.drain( .. )
                                                .map(|x| x - cursor.node_offset)
                                                .collect();
            right_line_offsets.insert(0, 0);

            // construct left and right nodes
            let left_node = BufferNode::new(
                                from,
                                index,
                                cursor.node_offset,
                                left_line_offsets
                            );
            let right_node = BufferNode::new(
                                from,
                                index + cursor.node_offset,
                                offset - cursor.node_offset,
                                right_line_offsets
                            );

            // add the left, mid, right nodes in in that order to the list
            //  then remove the node that was split into those three
            self.node_list.insert(cursor.node_idx, left_node);
            self.node_list.insert(cursor.node_idx + 1, new_node);
            self.node_list.insert(cursor.node_idx + 2, right_node);
            self.node_list.remove(cursor.node_idx + 3);

            // update cursor to point at the beginning of the right node
            cursor.node_idx += 2;
            cursor.line_offset = 0;
            cursor.node_offset = 0;
            cursor.line_idx = 0;
        }
    }

    pub fn remove(&mut self, cursor: &mut Cursor) {

        if self.node_list.len() == 0 {
            // there's nothing to delete so just return
            return;
        }

        if cursor.node_idx == 0 && cursor.node_offset == 0 {
            // cursor is all the way at the front of the buffer
            //  so just return
            return;
        }

        let node_idx = cursor.node_idx;
        let node_offset = cursor.node_offset;
        if node_offset == 0 {
            // cursor should be in the front of a node
            //  reduce the offset of the previous node by 1
            //  then update cursor and node_list as necessary

            // get the previous node
            let prev_node = &mut self.node_list[node_idx - 1];

            if prev_node.last_line_offset() == prev_node.offset() {
                assert!(prev_node.offset() >= 1);
                // the last character of that node should be '\n'
                //  in that case we should remove it from the list
                //  of line offsets
                prev_node.line_offsets.pop();
            }

            prev_node.reduce_offset_by(1);

            if prev_node.offset == 0 {
                // node should now be empty
                //  remove it from list
                cursor.node_idx -= 1;
                self.node_list.remove(cursor.node_idx);
            }
            // no changes should need to be made to the cursor
            //  for line_offsets, line_idx and node_offset
            //  since it should still be referring to the same node
        } else if node_offset == self.node_list[node_idx].offset() {
            // cursor should be at the end of a node
            //  should only happen when cursor is at the last node
            assert_eq!(node_idx, self.node_list.len() - 1);

            let current_node = &mut self.node_list[cursor.node_idx];
            if current_node.last_line_offset() == cursor.node_offset {
                // the last character of current node should be '\n'
                current_node.line_offsets.pop();
            }

            current_node.reduce_offset_by(1);

            if current_node.offset() == 0 {
                // node should now be empty, remove it from list
                self.node_list.remove(cursor.node_idx);

                // refer to previous node if it is not the first node
                cursor.node_idx = match cursor.node_idx {
                    0 => 0,
                    x => x - 1,
                };

                if self.node_list.len() == 0 {
                    // should not be any more nodes in node_list
                    //  zero out the cursor and return
                    cursor.node_idx = 0;
                    cursor.node_offset = 0;
                    cursor.line_idx = 0;
                    cursor.line_offset = 0;
                    return;
                }

                // update cursor to point to the last position in the previous node
                let current_node = &self.node_list[cursor.node_idx];
                cursor.node_offset = current_node.offset();
                cursor.line_idx = current_node.line_offsets_len() - 1;
                cursor.line_offset = cursor.node_offset - current_node.last_line_offset();
            } else {
                // node should not be empty
                //  reduce offset of cursor by 1
                //  and update line_idx and line_offset accordingly
                cursor.node_offset -= 1;
                cursor.line_idx = current_node.line_offsets_len() - 1;
                cursor.line_offset = current_node.offset() - current_node.last_line_offset();
            }
        } else {
            // cursor should be in the middle of a node
            let current_node = &mut self.node_list[cursor.node_idx];
            let from = current_node.from();
            let index = current_node.index();
            let offset = current_node.offset();
            let line_offsets = current_node.line_offsets();

            // split the line offsets of the current node into two parts
            let mut left_line_offsets: Vec<usize> = line_offsets.drain(..=cursor.line_idx)
                                                        .collect();
            let mut right_line_offsets: Vec<usize> = line_offsets.drain( .. )
                                                        .map(|x| x - cursor.node_offset)
                                                        .collect();
            right_line_offsets.insert(0, 0);

            if *left_line_offsets.last().unwrap() == cursor.node_offset {
                // the last character of the left node should be '\n'
                left_line_offsets.pop();
            }

            let left_node = BufferNode::new(
                                from,
                                index,
                                index + cursor.node_offset - 1,
                                left_line_offsets
                            );
            let right_node = BufferNode::new(
                                from,
                                index + cursor.node_offset,
                                offset - cursor.node_offset,
                                right_line_offsets
                            );

            // insert both the left node and the right node
            //  and remove the node that was split
            self.node_list.insert(cursor.node_idx, left_node);
            self.node_list.insert(cursor.node_idx + 1, right_node);
            self.node_list.remove(cursor.node_idx + 2);

            // update cursor to point at the beginning of the right node
            cursor.node_idx += 1;
            cursor.node_offset = 0;
            cursor.line_idx = 0;
            cursor.line_offset = 0;
        }
    }

    pub fn remove_word(&mut self, cursor: &mut Cursor) -> char {
        todo!();
    }

    pub fn as_str(&self) -> String {
        // construct a serialised string by iterating through the nodes
        //  and copying the contents that they refer to based on
        //  their indices and offsets and which string they refer to
        self.node_list.iter()
            .fold(String::new(),
                |mut acc, node| {
                let source = match node.from() {
                    BufferType::Original => &self.original_str,
                    BufferType::Added => &self.added_str,
                };
                let slice = &source[node.index()..node.index() + node.offset()];
                let chunk = str::from_utf8(slice).unwrap();
                acc.push_str(chunk);
                acc
            }
        )
    }

    pub fn move_cursor_up(&self, cursor: &mut Cursor) {
        todo!();
    }

    pub fn move_cursor_down(&self, cursor: &mut Cursor) {
        todo!();
    }

    pub fn move_cursor_left(&self, cursor: &mut Cursor) {
        if cursor.node_idx == 0 && cursor.node_offset == 0 {
            // cursor should be at the beginning of buffer
            //  do nothing and return
            return;
        }

        if cursor.node_offset == 0 {
            // cursor should be at the front of a node
            //  move the cursor to the previous node
            //  update the node_offset and line_offset and line_idx
            //  based on the second last character
            cursor.node_idx -= 1;

            let current_node: &BufferNode = &self.node_list[cursor.node_idx];

            cursor.node_offset = current_node.offset() - 1;
            if current_node.last_line_offset() == current_node.offset() {
                // the last character of current node should be '\n'
                //	so set cursor to point at the second last line
                cursor.line_idx = current_node.line_offsets_len() - 2;
                cursor.line_offset = cursor.node_offset - current_node.line_offset_at(cursor.line_idx);
            } else {
                // the last character of current node should not '\n'
                //	so the cursor should point at the last line
                cursor.line_idx = current_node.line_offsets_len() - 1;
                cursor.line_offset = cursor.node_offset - current_node.line_offset_at(cursor.line_idx);
            }
        } else {
            // cursor is not at the beginning of the node
            cursor.node_offset -= 1;

            let current_node: &BufferNode = &self.node_list[cursor.node_idx];

            if cursor.line_offset == 0 {
                // cursor should have moved over a '\n'
                //	reduce the line index by 1, and update the line offset
                cursor.line_idx -= 1;
                cursor.line_offset = cursor.node_offset - current_node.line_offset_at(cursor.line_idx);
            } else {
                // the last character of current node should not be '\n'
                //	reduce the line offset by 1
                cursor.line_offset -= 1;
            }
        }
    }

    pub fn move_cursor_right(&self, cursor: &mut Cursor) {
        if self.node_list.len() == 0 {
            // node_list should be empty and thus there is nothing to do
            //	so just return
            return;
        }
        if cursor.node_idx == self.node_list.len() - 1
            && cursor.node_offset == self.node_list.last().unwrap().offset() {
                // cursor should be pointing at the last position in the buffer
                //	so there should be nothing to do but return
                return;
        }

        if cursor.node_offset + 1 == self.node_list.last().unwrap().offset() {
            // cursor should be at second last position of the current node
            if cursor.node_idx == self.node_list.len() - 1 {
                // cursor should be on the last node in the list
                //	so just increment the node_offset and line_offset
                cursor.node_offset += 1;
                let current_node = &self.node_list[cursor.node_idx];
                if cursor.node_offset == current_node.last_line_offset() {
                    // cursor should have passed over a '\n'
                    cursor.line_idx += 1;
                    cursor.line_offset = 0;
                } else {
                    // cursor should have passed over something else
                    cursor.line_offset += 1;
                }
            } else {
                // cursor should have a node on the right
                //	point the cursor to the beginning of that node
                cursor.node_idx += 1;
                cursor.node_offset = 0;
                cursor.line_idx = 0;
                cursor.line_offset = 0;
            }

        } else {
            // cursor should be still within the same node
            //	increase the node_offset by 1
            //	update the line index based on whether it has crossed
            cursor.node_offset += 1;
            let current_node = &self.node_list[cursor.node_idx];
            if cursor.node_idx < current_node.line_offsets_len() - 1
                &&	cursor.node_offset
                    == current_node.line_offset_at(cursor.line_idx + 1) {
                // cursor should have just passed over a '\n'
                cursor.line_idx += 1;
                cursor.line_offset = 0;
            } else {
                cursor.line_offset += 1;
            }
        }
    }

}

#[cfg(test)]
#[path = "tests/buffer_tests.rs"]
mod buffer_tests;