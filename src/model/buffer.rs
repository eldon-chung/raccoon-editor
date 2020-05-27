use std::str;

use crate::utils::cursor::Cursor;
use crate::utils::gapbuffer::GapBuffer;

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
        BufferNode {
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
    node_list: GapBuffer<BufferNode>,
    // an important invariant is how the cursor is going to be placed.
    //  this is going to be maintained throughout the operations.
    //  if the cursor is in between two nodes, it will always be at the 0 index of the
    //  node on the right, rather than the end index of the left node.
    //  the only time the cursor will be at the end index of a node is if there is
    //  no other node to its right.
}

#[allow(dead_code)]
impl Buffer {
    fn get_offsets(string: &str) -> Vec<usize> {
        let acc = vec![0];
        let index_list: Vec<usize> = (1..=string.len()).collect();
        string
            .chars()
            .zip(index_list.iter())
            .filter(|(c, _)| *c == '\n')
            .fold(acc, |mut list, (_c, idx)| {
                list.push(*idx);
                list
            })
    }

    pub fn new() -> Buffer {
        Buffer {
            original_str: Vec::new(),
            added_str: Vec::new(),
            node_list: GapBuffer::new(),
        }
    }

    pub fn with_contents(original_str: String) -> Buffer {
        let offsets = Buffer::get_offsets(&original_str);
        let first_node = BufferNode::new(BufferType::Original, 0, original_str.len(), offsets);
        Buffer {
            original_str: original_str.as_bytes().to_vec(),
            added_str: Vec::new(),
            node_list: GapBuffer::with_contents(vec![first_node]),
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

        // get the fields of the node
        assert!(new_node.line_offsets_len() >= 1);
        let offset = new_node.offset();
        let last_line_offset = new_node.last_line_offset();

        // construct closure on how to update line_offset when it comes to it
        let line_idx_update: Box<dyn Fn(usize) -> usize> = match new_node.line_offsets_len() {
            1 => Box::new(|line_offset| line_offset + offset),
            _ => Box::new(|line_offset| offset - last_line_offset),
        };

        // append string to add_str
        let mut vec_converted = string.as_bytes().to_vec();
        self.added_str.append(&mut vec_converted);

        if self.node_list.is_empty() {
            // node_list should be empty, so just push and return
            //  after insertion cursor should be referring to the end of the first node
            assert_eq!(cursor.node_offset, 0);
            assert_eq!(cursor.line_idx, 0);
            assert_eq!(cursor.line_offset, 0);
            assert_eq!(cursor.original_line_offset, 0);

            cursor.node_offset = new_node.offset();
            cursor.line_idx = new_node.line_offsets_len() - 1;
            cursor.line_offset = line_idx_update(cursor.line_offset);
            cursor.original_line_offset = cursor.line_offset;
            self.node_list.insert_before(new_node);
            return;
        }

        let node_to_split: &mut BufferNode = self.node_list.get_current_mut().unwrap();
        if cursor.node_offset == node_to_split.offset() {
            // cursor should be at the end of the buffer
            assert!(self.node_list.is_tail());
            // push the new node at the end of node_list
            //  update cursor to point at the end of the new node
            cursor.node_offset = new_node.offset();
            cursor.line_idx = new_node.line_offsets_len() - 1;
            cursor.line_offset = line_idx_update(cursor.line_offset);
            cursor.original_line_offset = cursor.line_offset;

            self.node_list.insert_after(new_node);
            self.node_list.move_pointer_right();
        } else if cursor.node_offset == 0 {
            // cursor should be at the front of a node
            assert_eq!(cursor.node_offset, 0);
            assert_eq!(cursor.line_idx, 0);
            // just insert the new node before current node
            //	which is where the cursor is
            self.node_list.insert_before(new_node);
            cursor.node_idx += 1;
            cursor.line_offset = line_idx_update(cursor.line_offset);
            cursor.original_line_offset = cursor.line_offset;
        } else {
            // split the node into two and replace the current node with them
            let from = node_to_split.from();
            let index = node_to_split.index();
            let offset = node_to_split.offset();
            let line_offsets = &mut node_to_split.line_offsets();

            // split the line_offsets based on where the cursor is
            let left_line_offsets = line_offsets.drain(..=cursor.line_idx).collect();
            let mut right_line_offsets: Vec<usize> = line_offsets
                .drain(..)
                .map(|x| x - cursor.node_offset)
                .collect();
            right_line_offsets.insert(0, 0);

            // construct left and right nodes
            let left_node = BufferNode::new(from, index, cursor.node_offset, left_line_offsets);
            let right_node = BufferNode::new(
                from,
                index + cursor.node_offset,
                offset - cursor.node_offset,
                right_line_offsets,
            );

            // add the left, mid, right nodes in in that order to the list
            //  then remove the node that was split into those three
            self.node_list.insert_before(left_node);
            self.node_list.insert_before(new_node);
            self.node_list.insert_before(right_node);
            self.node_list.delete_current();

            // update cursor to point at the beginning of the right node
            cursor.node_offset = 0;
            cursor.line_idx = 0;
            cursor.line_offset = line_idx_update(cursor.line_offset);
            cursor.original_line_offset = cursor.line_offset;
        }
    }

    pub fn remove(&mut self, cursor: &mut Cursor) {
        
        if self.node_list.is_empty() {
            // node_list should be empty, so just return
            assert_eq!(cursor.node_offset, 0);
            assert_eq!(cursor.line_idx, 0);
            assert_eq!(cursor.line_offset, 0);
            assert_eq!(cursor.original_line_offset, 0);
            return;
        }

        if self.node_list.is_head() && cursor.node_offset == 0 {
            // cursor should just be at the end of the buffer
            //  so just return
            return;
        }

        let node_idx = cursor.node_idx;
        let node_offset = cursor.node_offset;
        if node_offset == 0 {
            // cursor should be in the front of a node
            //  reduce the offset of the previous node by 1
            //  then update cursor and node_list as necessary

            // get fields of the previous node
            let prev_last_line_offset = self.node_list.get_prev().unwrap().last_line_offset();
            let prev_offset = self.node_list.get_prev().unwrap().offset();
            let prev_line_offsets = self.node_list.get_prev_mut().unwrap().line_offsets();
            let mut was_newline = false;
            if prev_last_line_offset == prev_offset {
                assert!(prev_offset >= 1);
                // the last character of that node should be '\n'
                //  in that case we should remove it from the list
                //  of line offsets
                prev_line_offsets.pop();
                was_newline = true;
            }

            self.node_list.get_prev_mut().unwrap().reduce_offset_by(1);

            if self.node_list.get_prev().unwrap().offset() == 0 {
                // previous node should be empty
                //  remove previous node from list
                self.node_list.delete_before();
            }
            // no changes should need to be made to the cursor for node_offset
            //  and line_idx since cursor should still be on the same node
            // update line_offset based on whether the character deleted was a newline
            assert_eq!(was_newline, cursor.line_offset == 0);
            if was_newline {
                // deleted character should have been a newline
                //  recompute the line_offset starting from previous node
                let mut to_add: usize = 0;
                for indexed_node in self.node_list.left_list_as_vec().iter().rev().skip(1) {
                    to_add += indexed_node.offset() - indexed_node.last_line_offset();
                    if indexed_node.last_line_offset() != 0 {
                        break;
                    }
                }
                cursor.line_offset = to_add;
                cursor.original_line_offset = cursor.line_offset;
            } else {
                // deleted character should not have been a newline
                cursor.line_offset -= 1;
                cursor.original_line_offset = cursor.line_offset;
            }
        } else if node_offset == self.node_list.get_current().unwrap().offset() {
            // cursor should be at the end of a node
            //  should only happen when cursor is at the last node
            assert!(self.node_list.is_tail());
            // get fields of the current node
            let curr_last_line_offset = self.node_list.get_current().unwrap().last_line_offset();
            let curr_offset = self.node_list.get_current().unwrap().offset();
            let curr_line_offsets = self.node_list.get_current_mut().unwrap().line_offsets();
            let mut was_newline = false;
            if curr_last_line_offset == cursor.node_offset {
                // the last character of current node should be '\n'
                curr_line_offsets.pop();
                was_newline = true;
            }

            self.node_list
                .get_current_mut()
                .unwrap()
                .reduce_offset_by(1);

            if self.node_list.get_current().unwrap().offset() == 0 {
                // node should now be empty, remove it from list
                //  then refer to previous node if it is not the first node
                self.node_list.delete_current();

                if self.node_list.len() == 0 {
                    // should not be any more nodes in node_list
                    //  zero out the cursor and return
                    cursor.node_idx = 0;
                    cursor.node_offset = 0;
                    cursor.line_idx = 0;
                    cursor.line_offset = 0;
                    cursor.original_line_offset = 0;
                    return;
                }

                // update cursor to point to the last position in the previous node
                let current_node = self.node_list.get_current().unwrap();
                cursor.node_offset = current_node.offset();
                cursor.line_idx = current_node.line_offsets_len() - 1;
            } else {
                // node should not be empty
                //  reduce offset of cursor by 1
                //  and update line_idx and line_offset accordingly
                let current_node = &self.node_list.get_current().unwrap();
                cursor.node_offset -= 1;
                cursor.line_idx = current_node.line_offsets_len() - 1;
            }

            assert_eq!(was_newline, cursor.line_offset == 0);
            if was_newline {
                // deleted character should be a newline
                //  starting from current node, iterate back and find the newline
                let mut to_add: usize = 0;
                for indexed_node in self.node_list.left_list_as_vec().iter().rev() {
                    to_add += indexed_node.offset() - indexed_node.last_line_offset();
                    if indexed_node.last_line_offset() != 0 {
                        break;
                    }
                }
                cursor.line_offset = to_add;
                cursor.original_line_offset = cursor.line_offset;
            } else {
                // deleted character should not be a newline
                cursor.line_offset -= 1;
                cursor.original_line_offset = cursor.line_offset;
            }
        } else {
            // cursor should be in the middle of a node
            let from = self.node_list.get_current().unwrap().from();
            let index = self.node_list.get_current().unwrap().index();
            let offset = self.node_list.get_current().unwrap().offset();
            let line_offsets = self.node_list.get_current_mut().unwrap().line_offsets();
            let mut was_newline = false;
            // split the line offsets of the current node into two parts
            let mut left_line_offsets: Vec<usize> =
                line_offsets.drain(..=cursor.line_idx).collect();
            let mut right_line_offsets: Vec<usize> = line_offsets
                .drain(..)
                .map(|x| x - cursor.node_offset)
                .collect();
            right_line_offsets.insert(0, 0);

            if *left_line_offsets.last().unwrap() == cursor.node_offset {
                // the last character of the left node should be '\n'
                left_line_offsets.pop();
                was_newline = true;
            }

            let left_node = BufferNode::new(
                from,
                index,
                index + cursor.node_offset - 1,
                left_line_offsets,
            );
            let right_node = BufferNode::new(
                from,
                index + cursor.node_offset,
                offset - cursor.node_offset,
                right_line_offsets,
            );

            // insert both the left node and the right node
            //  and remove the node that was split
            self.node_list.insert_after(left_node);
            self.node_list.delete_current();
            self.node_list.move_pointer_right();
            self.node_list.insert_after(right_node);

            // update cursor to point at the beginning of the right node
            self.node_list.move_pointer_right();
            cursor.node_offset = 0;
            cursor.line_idx = 0;

            assert_eq!(was_newline, cursor.line_offset == 0);
            if was_newline {
                // deleted character should be a newline
                //  starting from previous node calculate the line_offset
                let mut to_add: usize = 0;
                for indexed_node in self.node_list.left_list_as_vec().iter().rev().skip(1) {
                    to_add += indexed_node.offset() - indexed_node.last_line_offset();
                    if indexed_node.last_line_offset() != 0 {
                        break;
                    }
                }
                cursor.line_offset = to_add;
                cursor.original_line_offset = cursor.line_offset;
            } else {
                // deleted characted should not be a newline
                cursor.line_offset -= 1;
                cursor.original_line_offset = cursor.line_offset;
            }
        }
    }

    pub fn as_str(&self) -> String {
        // construct a serialised string by iterating through the nodes
        //  and copying the contents that they refer to based on
        //  their indices and offsets and which string they refer to
        self.node_list
            .left_right_list_as_vec()
            .iter()
            .fold(String::new(), |mut acc, node| {
                let source = match node.from() {
                    BufferType::Original => &self.original_str,
                    BufferType::Added => &self.added_str,
                };
                let slice = &source[node.index()..node.index() + node.offset()];
                let chunk = str::from_utf8(slice).unwrap();
                acc.push_str(chunk);
                acc
            })
    }

    pub fn move_cursor_up(&self, cursor: &mut Cursor) {
        todo!();
    }

    pub fn move_cursor_down(&self, cursor: &mut Cursor) {
        todo!();
    }

    pub fn move_cursor_left(&mut self, cursor: &mut Cursor) {
        if self.node_list.is_empty() {
            // buffer should be empty
            //  do nothing and return
            return;
        }

        if self.node_list.is_head() && cursor.node_offset == 0 {
            // cursor should be at the beginning of buffer
            //  do nothing and return
            cursor.original_line_offset = cursor.line_offset;
            return;
        }

        if cursor.node_offset == 0 {
            // cursor should be at the front of a node
            //  move the cursor to the previous node
            // update the node_offset and line_offset and line_idx
            //  based on the last character
            self.node_list.move_pointer_left();
            let current_node: &BufferNode = self.node_list.get_current().unwrap();

            cursor.node_offset = current_node.offset() - 1;
            if current_node.last_line_offset() == current_node.offset() {
                // the last character of current node should be '\n'
                //	set cursor to point at the second last line
                //  and recompute the line offset
                let line_idx = current_node.line_offsets_len() - 2;
                if line_idx != 0 {
                    // should have a '\n' before the cursor in the current node
                    cursor.line_idx = line_idx;
                    cursor.line_offset =
                        current_node.last_line_offset() - current_node.line_offset_at(line_idx) - 1;
                    cursor.original_line_offset = cursor.line_offset;
                } else {
                    // should not have a '\n' before the cursor in the current node
                    //  starting from the previous node
                    let mut to_add: usize = cursor.node_offset;
                    for indexed_node in self.node_list.left_list_as_vec().iter().rev().skip(1) {
                        to_add += indexed_node.offset() - indexed_node.last_line_offset();
                        if indexed_node.last_line_offset() != 0 {
                            break;
                        }
                    }
                    cursor.line_offset = to_add;
                    cursor.original_line_offset = cursor.line_offset;
                }
            } else {
                // the last character of current node should not be '\n'
                //	so the cursor should point at the last line
                cursor.line_idx = current_node.line_offsets_len() - 1;
                cursor.line_offset -= 1;
                cursor.original_line_offset = cursor.node_offset;
            }
        } else {
            // cursor is not at the beginning of the node
            cursor.node_offset -= 1;

            let current_node: &BufferNode = self.node_list.get_current().unwrap();
            if cursor.line_offset == 0 {
                // cursor should have moved over a '\n'
                //	reduce the line index by 1, and update the line offset
                cursor.line_idx -= 1;

                if cursor.line_idx != 0 {
                    // should have a '\n' before the cursor in the current node
                    cursor.line_offset = current_node.last_line_offset()
                        - current_node.line_offset_at(cursor.line_idx)
                        - 1;
                    cursor.original_line_offset = cursor.line_offset;
                } else {
                    // should not have a '\n' before the cursor in the current node
                    //  starting from the previous node get the offset
                    let mut to_add: usize = cursor.node_offset;
                    for indexed_node in self.node_list.left_list_as_vec().iter().rev().skip(1) {
                        to_add += indexed_node.offset() - indexed_node.last_line_offset();
                        if indexed_node.last_line_offset() != 0 {
                            break;
                        }
                    }
                    cursor.line_offset = to_add;
                    cursor.original_line_offset = cursor.line_offset;
                }
            } else {
                // the last character of current node should not be '\n'
                //	reduce the line offset by 1
                cursor.line_offset -= 1;
                cursor.original_line_offset = cursor.line_offset;
            }
        }
    }

    pub fn move_cursor_right(&mut self, cursor: &mut Cursor) {
        if self.node_list.is_empty() {
            // node_list should be empty and thus there is nothing to do
            //  so just return
            return;
        }

        if self.node_list.is_tail()
            && cursor.node_offset == self.node_list.get_current().unwrap().offset()
        {
            // cursor should be pointing at the last position in the buffer
            //  so there should be nothing to do but return
            cursor.original_line_offset = cursor.line_offset;
            return;
        }

        if cursor.node_offset + 1 == self.node_list.get_current().unwrap().offset() {
            // cursor should be at second last position of the current node
            if self.node_list.is_tail() {
                // cursor should be on the last node in the list
                //	so just increment the node_offset and line_offset
                cursor.node_offset += 1;
                let current_node = &self.node_list.get_current().unwrap();
                if cursor.node_offset == current_node.last_line_offset() {
                    // cursor should have passed over a '\n'
                    cursor.line_idx += 1;
                    cursor.line_offset = 0;
                    cursor.original_line_offset = cursor.line_offset;
                } else {
                    // cursor should have passed over something else
                    cursor.line_offset += 1;
                    cursor.original_line_offset = cursor.line_offset;
                }
            } else {
                // cursor should have a node on the right
                //	point the cursor to the beginning of that node
                self.node_list.move_pointer_right();
                cursor.node_offset = 0;
                cursor.line_idx = 0;
                if self.node_list.get_prev().unwrap().last_line_offset()
                    == self.node_list.get_prev().unwrap().offset()
                {
                    // cursor should have passed over a newline
                    cursor.line_offset = 0;
                    cursor.original_line_offset = cursor.line_offset;
                } else {
                    // cursor should not have passed over a newline
                    cursor.line_offset += 1;
                    cursor.original_line_offset = cursor.line_offset;
                }
            }
        } else {
            // cursor should be still within the same node
            //	increase the node_offset by 1
            //	update the line index based on whether it has crossed
            //  over a newline
            cursor.node_offset += 1;
            let current_node = &self.node_list.get_current().unwrap();
            if cursor.line_idx + 1 <= current_node.line_offsets_len() - 1
                && cursor.node_offset == current_node.line_offset_at(cursor.line_idx + 1)
            {
                // cursor should have just passed over a '\n'
                cursor.line_idx += 1;
                cursor.line_offset = 0;
                cursor.original_line_offset = cursor.line_offset;
            } else {
                cursor.line_offset += 1;
                cursor.original_line_offset = cursor.line_offset;
            }
        }
    }
}

#[cfg(test)]
#[path = "tests/buffer_tests.rs"]
mod buffer_tests;
