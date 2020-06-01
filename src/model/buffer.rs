use std::str;

use super::cursor::Cursor;
use super::nodelist::*;

macro_rules! min {
    ($x:expr, $y:expr) => {
        if $x < $y {
            $x
        } else {
            $y
        };
    };
}

pub struct Buffer {
    cursor: Cursor,
    original_str: Vec<u8>,
    added_str: Vec<u8>,
    node_list: NodeList,
    num_newlines: usize,
    current_line: usize,
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
            cursor: Cursor::new(),
            original_str: Vec::new(),
            added_str: Vec::new(),
            node_list: NodeList::new(),
            num_newlines: 0,
            current_line: 0,
        }
    }

    pub fn with_contents(original_str: String) -> Buffer {
        let offsets = Buffer::get_offsets(&original_str);
        let first_node = BufferNode::new(BufferType::Original, 0, original_str.len(), offsets);
        Buffer {
            cursor: Cursor::new(),
            original_str: original_str.as_bytes().to_vec(),
            added_str: Vec::new(),
            num_newlines: first_node.line_offsets_len() - 1,
            node_list: NodeList::with_contents(vec![first_node]),
            current_line: 0,
        }
    }

    pub fn current_line(&self) -> usize {
        self.current_line
    }

    pub fn num_newlines(&self) -> usize {
        self.num_newlines
    }

    pub fn insert(&mut self, ch: char) {
        let mut string = String::new();
        string.push(ch);
        self.insert_str(string);
    }

    pub fn insert_str(&mut self, string: String) {
        // update the number of newlines
        let line_offsets = Buffer::get_offsets(&string);
        self.num_newlines += line_offsets.len() - 1;
        self.current_line += line_offsets.len() - 1;

        // construct the new node to be inserted
        let idx = self.added_str.len();
        let offset = string.len();
        let new_node = BufferNode::new(BufferType::Added, idx, offset, line_offsets);

        // get the fields of the node
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
            assert_eq!(self.cursor.node_offset, 0);
            assert_eq!(self.cursor.line_idx, 0);
            assert_eq!(self.cursor.line_offset, 0);
            assert_eq!(self.cursor.original_line_offset, 0);

            self.cursor.node_offset = new_node.offset();
            self.cursor.line_idx = new_node.line_offsets_len() - 1;
            self.cursor.line_offset = line_idx_update(self.cursor.line_offset);
            self.cursor.original_line_offset = self.cursor.line_offset;
            self.node_list.insert_curr(new_node);
            return;
        }

        let node_to_split: &mut BufferNode = self.node_list.get_curr_mut();
        if self.cursor.node_offset == node_to_split.offset() {
            // cursor should be at the end of the buffer
            assert!(self.node_list.at_tail());
            // push the new node at the end of node_list
            //  update cursor to point at the end of the new node
            self.cursor.node_offset = new_node.offset();
            self.cursor.line_idx = new_node.line_offsets_len() - 1;
            self.cursor.line_offset = line_idx_update(self.cursor.line_offset);
            self.cursor.original_line_offset = self.cursor.line_offset;

            self.node_list.insert_curr(new_node);
        } else if self.cursor.node_offset == 0 {
            // cursor should be at the front of a node
            assert_eq!(self.cursor.node_offset, 0);
            assert_eq!(self.cursor.line_idx, 0);
            // just insert the new node before current node
            //	which is where the cursor is
            self.cursor.node_offset = new_node.offset();
            self.cursor.line_idx = new_node.line_offsets_len() - 1;
            self.cursor.line_offset = line_idx_update(self.cursor.line_offset);
            self.cursor.original_line_offset = self.cursor.line_offset;
            self.node_list.insert_prev(new_node);
        } else {
            // split the node into two and replace the current node with them
            let from = node_to_split.from();
            let index = node_to_split.index();
            let offset = node_to_split.offset();
            let line_offsets = &mut node_to_split.line_offsets();

            // split the line_offsets based on where the cursor is
            let cursor_node_offset = self.cursor.node_offset;
            let left_line_offsets = line_offsets.drain(..=self.cursor.line_idx).collect();
            let mut right_line_offsets: Vec<usize> = line_offsets
                .drain(..)
                .map(|x| x - cursor_node_offset)
                .collect();
            right_line_offsets.insert(0, 0);

            // construct left and right nodes
            let left_node = BufferNode::new(from, index, cursor_node_offset, left_line_offsets);
            let right_node = BufferNode::new(
                from,
                index + self.cursor.node_offset,
                offset - self.cursor.node_offset,
                right_line_offsets,
            );

            // add the left, mid, right nodes in in that order to the list
            //  then remove the node that was split into those three
            self.node_list.remove_curr();
            self.node_list.insert_curr(left_node);
            self.node_list.insert_curr(new_node);
            self.node_list.insert_curr(right_node);

            // update cursor to point at the beginning of the right node
            self.cursor.node_offset = 0;
            self.cursor.line_idx = 0;
            self.cursor.line_offset = line_idx_update(self.cursor.line_offset);
            self.cursor.original_line_offset = self.cursor.line_offset;
        }
    }

    pub fn remove(&mut self) {
        if self.node_list.is_empty() {
            // node_list should be empty, so just return
            assert_eq!(self.current_line, 0);
            assert_eq!(self.cursor.node_offset, 0);
            assert_eq!(self.cursor.line_idx, 0);
            assert_eq!(self.cursor.line_offset, 0);
            assert_eq!(self.cursor.original_line_offset, 0);
            return;
        }

        if self.node_list.at_head() && self.cursor.node_offset == 0 {
            // cursor should just be at the end of the buffer
            //  so just return
            return;
        }

        let node_offset = self.cursor.node_offset;
        if node_offset == 0 {
            // cursor should be in the front of a node
            //  reduce the offset of the previous node by 1
            //  then update cursor and node_list as necessary

            // get fields of the previous node
            let new_offset = self.node_list.get_prev().offset() - 1;
            let new_index = self.node_list.get_prev().index();
            let new_from = self.node_list.get_prev().from();
            let mut prev_line_offsets = self.node_list.get_prev_mut().line_offsets();
            let mut new_line_offsets: Vec<usize> = prev_line_offsets.drain(..).collect();

            let mut was_newline = false;
            if self.cursor.line_offset == 0 {
                // the last character of that node should be '\n'
                //  in that case we should remove it from the list
                //  of line offsets
                new_line_offsets.pop();
                was_newline = true;
            }

            // replace the previous node with a new one
            let new_node = BufferNode::new(new_from, new_index, new_offset, new_line_offsets);
            self.node_list.remove_prev();

            if new_offset > 0 {
                // previous node should not be empty after removal
                //  add new_node to list
                self.node_list.insert_prev(new_node);
            }
            // no changes should need to be made to the cursor for node_offset
            //  and line_idx since cursor should still be on the same node
            // update line_offset based on whether the character deleted was a newline
            assert_eq!(was_newline, self.cursor.line_offset == 0);
            if was_newline {
                // deleted character should have been a newline
                //  recompute the line_offset starting from previous node
                self.num_newlines -= 1;
                self.current_line -= 1;

                let mut to_add: usize = 0;
                for idx in (0..self.node_list.index()).rev() {
                    to_add += self.node_list.get(idx).offset()
                        - self.node_list.get(idx).last_line_offset();
                    if self.node_list.get(idx).has_newline() {
                        break;
                    }
                }
                self.cursor.line_offset = to_add;
                self.cursor.original_line_offset = self.cursor.line_offset;
            } else {
                // deleted character should not have been a newline
                self.cursor.line_offset -= 1;
                self.cursor.original_line_offset = self.cursor.line_offset;
            }
        } else if node_offset == self.node_list.get_curr().offset() {
            // cursor should be at the end of a node
            //  should only happen when cursor is at the last node
            assert!(self.node_list.at_tail());
            // get fields of the current node
            let new_from = self.node_list.get_curr_mut().from();
            let new_index = self.node_list.get_curr_mut().index();
            let new_offset = self.node_list.get_curr_mut().offset() - 1;
            let curr_line_offsets = self.node_list.get_curr_mut().line_offsets();
            let mut new_line_offsets: Vec<usize> = curr_line_offsets.drain(..).collect();

            let mut was_newline = false;
            if self.cursor.line_offset == 0 {
                // the last character of current node should be '\n'
                new_line_offsets.pop();
                was_newline = true;
            }

            // replace the current node
            let new_node = BufferNode::new(new_from, new_index, new_offset, new_line_offsets);
            self.node_list.insert_prev(new_node);
            self.node_list.remove_curr();

            if self.node_list.get_curr().offset() == 0 {
                // node should now be empty, remove it from list
                //  then refer to previous node if it is not the first node
                self.node_list.remove_curr();

                if self.node_list.is_empty() {
                    // should not be any more nodes in node_list
                    //  zero out the cursor and return
                    self.cursor.node_offset = 0;
                    self.cursor.line_idx = 0;
                    self.cursor.line_offset = 0;
                    self.cursor.original_line_offset = 0;
                    return;
                }

                // update cursor to point to the last position in the previous node
                self.cursor.node_offset = self.node_list.get_curr().offset();
                self.cursor.line_idx = self.node_list.get_curr().line_offsets_len() - 1;
            } else {
                // node should not be empty
                //  reduce offset of cursor by 1
                //  and update line_idx and line_offset accordingly
                self.cursor.node_offset -= 1;
                self.cursor.line_idx = self.node_list.get_curr().line_offsets_len() - 1;
            }

            assert_eq!(was_newline, self.cursor.line_offset == 0);
            if was_newline {
                // deleted character should be a newline
                //  starting from current node, iterate back and find the newline

                self.num_newlines -= 1;
                self.current_line -= 1;

                let mut to_add: usize = 0;
                for idx in (0..=self.node_list.index()).rev() {
                    to_add += self.node_list.get(idx).offset()
                        - self.node_list.get(idx).last_line_offset();
                    if self.node_list.get(idx).has_newline() {
                        break;
                    }
                }
                self.cursor.line_offset = to_add;
                self.cursor.original_line_offset = self.cursor.line_offset;
            } else {
                // deleted character should not be a newline
                self.cursor.line_offset -= 1;
                self.cursor.original_line_offset = self.cursor.line_offset;
            }
        } else {
            // cursor should be in the middle of a node
            let from = self.node_list.get_curr().from();
            let index = self.node_list.get_curr().index();
            let offset = self.node_list.get_curr().offset();
            let line_offsets = self.node_list.get_curr_mut().line_offsets();
            let mut was_newline = false;
            // split the line offsets of the current node into two parts
            let cursor_node_offset = self.cursor.node_offset;
            let mut left_line_offsets: Vec<usize> =
                line_offsets.drain(..=self.cursor.line_idx).collect();
            let mut right_line_offsets: Vec<usize> = line_offsets
                .drain(..)
                .map(|x| x - cursor_node_offset)
                .collect();
            right_line_offsets.insert(0, 0);

            if self.cursor.line_offset == 0 {
                // the last character of the left node should be '\n'
                left_line_offsets.pop();
                was_newline = true;
            }

            let left_node = BufferNode::new(from, index, cursor_node_offset - 1, left_line_offsets);
            let right_node = BufferNode::new(
                from,
                index + self.cursor.node_offset,
                offset - self.cursor.node_offset,
                right_line_offsets,
            );

            // insert both the left node and the right node
            //  and remove the node that was split
            if left_node.offset() > 0 {
                self.node_list.insert_prev(left_node);
            }
            self.node_list.insert_prev(right_node);
            self.node_list.remove_curr();

            // update cursor to point at the beginning of the right node
            self.cursor.node_offset = 0;
            self.cursor.line_idx = 0;

            assert_eq!(was_newline, self.cursor.line_offset == 0);
            if was_newline {
                // deleted character should be a newline
                //  starting from previous node calculate the line_offset

                self.num_newlines -= 1;
                self.current_line -= 1;

                let mut to_add: usize = 0;
                for idx in (0..self.node_list.index()).rev() {
                    to_add += self.node_list.get(idx).offset()
                        - self.node_list.get(idx).last_line_offset();
                    if self.node_list.get(idx).has_newline() {
                        break;
                    }
                }
                self.cursor.line_offset = to_add;
                self.cursor.original_line_offset = self.cursor.line_offset;
            } else {
                // deleted characted should not be a newline
                self.cursor.line_offset -= 1;
                self.cursor.original_line_offset = self.cursor.line_offset;
            }
        }
    }

    pub fn as_str(&self) -> String {
        // construct a serialised string by iterating through the nodes
        //  and copying the contents that they refer to based on
        //  their indices and offsets and which string they refer to
        self.node_list.iter().fold(String::new(), |mut acc, node| {
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

    pub fn as_str_split_by_cursors(&self) -> Vec<String> {
        // construct a serialised string by iterating through the nodes
        //  and copying the contents that they refer to based on
        //  their indices and offsets and which string they refer to

        if self.node_list.is_empty() {
            return vec![String::new()];
        }

        let left_iter = self.node_list.iter_until_curr();
        let right_iter = self.node_list.iter_from_after_curr();
        let mut left_str = String::new();
        let mut right_str = String::new();

        let mut left_str = left_iter
            .enumerate()
            .filter_map(|(i, e)| {
                if i != self.node_list.index() {
                    Some(e)
                } else {
                    None
                }
            })
            .fold(left_str, |mut acc, node| {
                let source = match node.from() {
                    BufferType::Original => &self.original_str,
                    BufferType::Added => &self.added_str,
                };
                let slice = &source[node.index()..node.index() + node.offset()];
                let chunk = str::from_utf8(slice).unwrap();
                acc.push_str(chunk);
                acc
            });
        let current_node = self.node_list.get_curr();
        let source = match current_node.from() {
            BufferType::Original => &self.original_str,
            BufferType::Added => &self.added_str,
        };
        let slice = &source[current_node.index()..current_node.index() + self.cursor.node_offset];
        let chunk = str::from_utf8(slice).unwrap();
        left_str.push_str(chunk);

        let slice = &source[current_node.index() + self.cursor.node_offset..current_node.offset()];
        let chunk = str::from_utf8(slice).unwrap();
        right_str.push_str(chunk);

        let right_str = right_iter.fold(right_str, |mut acc, node| {
            let source = match node.from() {
                BufferType::Original => &self.original_str,
                BufferType::Added => &self.added_str,
            };
            let slice = &source[node.index()..node.index() + node.offset()];
            let chunk = str::from_utf8(slice).unwrap();
            acc.push_str(chunk);
            acc
        });
        vec![left_str, right_str]
    }

    pub fn move_cursor_up(&mut self) {
        if self.current_line == 0 {
            return;
        }

        self.current_line -= 1;

        if self.cursor.line_idx == 0 {
            self.node_list.move_to_prev_newline();
            self.cursor.line_idx = self.node_list.get_curr().line_offsets_len() - 1;
            // guaranteed to find a node with a newline since if there were no more
            //  then self.current_line should be 0
        }

        if self.cursor.line_idx == 1 {
            self.node_list.move_to_prev_newline();
            let mut remaining_offset = self.cursor.original_line_offset;
            let max_line_offset =
                self.node_list.get_curr().offset() - self.node_list.get_curr().last_line_offset();
            if remaining_offset < max_line_offset {
                self.cursor.node_offset =
                    self.node_list.get_curr().last_line_offset() + remaining_offset;
                self.cursor.line_offset = remaining_offset;
                self.cursor.line_idx = self.node_list.get_curr().line_offsets_len() - 1;
                return;
            }
            remaining_offset -= max_line_offset;
            self.cursor.line_offset = max_line_offset;
            self.node_list.move_right();

            let mut max_line_offset = if self.node_list.get_curr().has_newline() {
                self.node_list.get_curr().line_offset_at(1)
            } else {
                self.node_list.get_curr().offset()
            };
            while !self.node_list.get_curr().has_newline()
                && !self.node_list.at_tail()
                && remaining_offset >= max_line_offset
            {
                self.cursor.line_offset += max_line_offset;
                remaining_offset -= max_line_offset;
                self.node_list.move_right();
                max_line_offset = if self.node_list.get_curr().has_newline() {
                    self.node_list.get_curr().line_offset_at(1)
                } else {
                    self.node_list.get_curr().offset()
                };
            }

            let max_line_offset = if self.node_list.get_curr().has_newline() {
                self.node_list.get_curr().line_offset_at(1) - 1
            } else {
                self.node_list.get_curr().offset()
            };
            let to_add = if max_line_offset < remaining_offset {
                max_line_offset
            } else {
                remaining_offset
            };
            self.cursor.node_offset = to_add;
            self.cursor.line_offset += to_add;
            self.cursor.line_idx = 0;
        } else {
            // self.cursor.line_idx should be greater than 1
            self.cursor.line_idx -= 1;
            let max_line_offset = self
                .node_list
                .get_curr()
                .line_offset_at(self.cursor.line_idx + 1)
                - self
                    .node_list
                    .get_curr()
                    .line_offset_at(self.cursor.line_idx)
                - 1;
            self.cursor.line_offset = if max_line_offset < self.cursor.original_line_offset {
                max_line_offset
            } else {
                self.cursor.original_line_offset
            };
            self.cursor.node_offset = self.cursor.line_offset
                + self
                    .node_list
                    .get_curr()
                    .line_offset_at(self.cursor.line_idx);
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.current_line == self.num_newlines {
            return;
        }

        self.current_line += 1;

        if self.cursor.line_idx == self.node_list.get_curr().line_offsets_len() - 1 {
            self.node_list.move_to_next_newline();
            self.cursor.line_idx = 0;
        }

        if self.cursor.line_idx + 1 == self.node_list.get_curr().line_offsets_len() - 1 {
            // second last
            let mut remaining_offset = self.cursor.original_line_offset;
            let max_line_offset =
                self.node_list.get_curr().offset() - self.node_list.get_curr().last_line_offset();
            if remaining_offset < max_line_offset || self.node_list.at_tail() {
                self.cursor.node_offset = min!(
                    self.node_list.get_curr().last_line_offset() + remaining_offset,
                    self.node_list.get_curr().offset()
                );
                self.cursor.line_offset = min!(
                    remaining_offset,
                    self.node_list.get_curr().offset()
                        - self.node_list.get_curr().last_line_offset()
                );
                self.cursor.line_idx = self.node_list.get_curr().line_offsets_len() - 1;
                return;
            }

            remaining_offset -= max_line_offset;
            self.cursor.line_offset = max_line_offset;
            self.node_list.move_right();

            let mut max_line_offset = if self.node_list.get_curr().has_newline() {
                self.node_list.get_curr().line_offset_at(1)
            } else {
                self.node_list.get_curr().offset()
            };

            while remaining_offset >= max_line_offset
                && !self.node_list.at_tail()
                && !self.node_list.get_curr().has_newline()
            {
                self.cursor.line_offset += max_line_offset;
                remaining_offset -= max_line_offset;
                self.node_list.move_right();
                max_line_offset = if self.node_list.get_curr().has_newline() {
                    self.node_list.get_curr().line_offset_at(1)
                } else {
                    self.node_list.get_curr().offset()
                };
            }

            let max_line_offset = if self.node_list.get_curr().has_newline() {
                self.node_list.get_curr().line_offset_at(1) - 1
            } else {
                self.node_list.get_curr().offset()
            };
            let to_add = min!(remaining_offset, max_line_offset);
            self.cursor.node_offset = to_add;
            self.cursor.line_offset += to_add;
            self.cursor.line_idx = 0;
        } else {
            // before second last
            assert!(self.cursor.line_idx + 2 <= self.node_list.get_curr().line_offsets_len() - 1);
            self.cursor.line_idx += 1;
            let max_line_offset = self
                .node_list
                .get_curr()
                .line_offset_at(self.cursor.line_idx + 1)
                - self
                    .node_list
                    .get_curr()
                    .line_offset_at(self.cursor.line_idx)
                - 1;
            self.cursor.line_offset = if max_line_offset < self.cursor.original_line_offset {
                max_line_offset
            } else {
                self.cursor.original_line_offset
            };
            self.cursor.node_offset = self.cursor.line_offset
                + self
                    .node_list
                    .get_curr()
                    .line_offset_at(self.cursor.line_idx);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.node_list.is_empty() {
            // buffer should be empty
            //  do nothing and return
            return;
        }

        if self.node_list.at_head() && self.cursor.node_offset == 0 {
            // cursor should be at the beginning of buffer
            //  do nothing and return
            self.cursor.original_line_offset = self.cursor.line_offset;
            return;
        }

        if self.cursor.node_offset == 0 {
            // cursor should be at the front of a node
            //  move the cursor to the previous node
            // update the node_offset and line_offset and line_idx
            //  based on the last character
            self.node_list.move_left();
            let current_node: &BufferNode = self.node_list.get_curr();

            self.cursor.node_offset = current_node.offset() - 1;
            if current_node.last_line_offset() == current_node.offset() {
                // the last character of current node should be '\n'
                //	set cursor to point at the second last line
                //  and recompute the line offset

                self.current_line -= 1;

                let line_idx = current_node.line_offsets_len() - 2;
                if line_idx > 0 {
                    // should have a '\n' before the cursor in the current node
                    self.cursor.line_idx = line_idx;
                    self.cursor.line_offset =
                        current_node.offset() - current_node.line_offset_at(line_idx) - 1;
                    self.cursor.original_line_offset = self.cursor.line_offset;
                } else {
                    // should not have a '\n' before the cursor in the current node
                    //  starting from the previous node
                    let mut to_add: usize = self.cursor.node_offset;
                    for idx in (0..self.node_list.index()).rev() {
                        to_add += self.node_list.get(idx).offset()
                            - self.node_list.get(idx).last_line_offset();
                        if self.node_list.get(idx).last_line_offset() != 0 {
                            break;
                        }
                    }
                    self.cursor.line_offset = to_add;
                    self.cursor.original_line_offset = self.cursor.line_offset;
                }
            } else {
                // the last character of current node should not be '\n'
                //	so the cursor should point at the last line
                self.cursor.line_idx = current_node.line_offsets_len() - 1;
                self.cursor.line_offset -= 1;
                self.cursor.original_line_offset = self.cursor.line_offset;
            }
        } else {
            // cursor is not at the beginning of the node
            self.cursor.node_offset -= 1;

            let current_node: &BufferNode = self.node_list.get_curr();
            if self.cursor.line_offset == 0 {
                // cursor should have moved over a '\n'
                //	reduce the line index by 1, and update the line offset
                self.current_line -= 1;
                self.cursor.line_idx -= 1;

                if self.cursor.line_idx > 0 {
                    // should have a '\n' before the cursor in the current node
                    self.cursor.line_offset = current_node.last_line_offset()
                        - current_node.line_offset_at(self.cursor.line_idx)
                        - 1;
                    self.cursor.original_line_offset = self.cursor.line_offset;
                } else {
                    // should not have a '\n' before the cursor in the current node
                    //  starting from the previous node get the offset
                    let mut to_add: usize = self.cursor.node_offset;
                    for idx in (0..self.node_list.index()).rev() {
                        to_add += self.node_list.get(idx).offset()
                            - self.node_list.get(idx).last_line_offset();
                        if self.node_list.get(idx).last_line_offset() != 0 {
                            break;
                        }
                    }
                    self.cursor.line_offset = to_add;
                    self.cursor.original_line_offset = self.cursor.line_offset;
                }
            } else {
                // the last character of current node should not be '\n'
                //	reduce the line offset by 1
                self.cursor.line_offset -= 1;
                self.cursor.original_line_offset = self.cursor.line_offset;
            }
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.node_list.is_empty() {
            // node_list should be empty and thus there is nothing to do
            //  so just return
            return;
        }

        if self.node_list.at_tail() && self.cursor.node_offset == self.node_list.get_curr().offset()
        {
            // cursor should be pointing at the last position in the buffer
            //  so there should be nothing to do but return
            self.cursor.original_line_offset = self.cursor.line_offset;
            return;
        }

        if self.cursor.node_offset + 1 == self.node_list.get_curr().offset() {
            // cursor should be at second last position of the current node
            if self.node_list.at_tail() {
                // cursor should be on the last node in the list
                //	so just increment the node_offset and line_offset
                self.cursor.node_offset += 1;
                let current_node = &self.node_list.get_curr();
                if self.cursor.node_offset == current_node.last_line_offset() {
                    // cursor should have passed over a '\n'
                    self.current_line += 1;
                    self.cursor.line_idx += 1;
                    self.cursor.line_offset = 0;
                    self.cursor.original_line_offset = self.cursor.line_offset;
                } else {
                    // cursor should have passed over something else
                    self.cursor.line_offset += 1;
                    self.cursor.original_line_offset = self.cursor.line_offset;
                }
            } else {
                // cursor should have a node on the right
                //	point the cursor to the beginning of that node
                self.node_list.move_right();
                self.cursor.node_offset = 0;
                self.cursor.line_idx = 0;
                if self.node_list.get_prev().last_line_offset()
                    == self.node_list.get_prev().offset()
                {
                    // cursor should have passed over a newline
                    self.current_line += 1;
                    self.cursor.line_offset = 0;
                    self.cursor.original_line_offset = self.cursor.line_offset;
                } else {
                    // cursor should not have passed over a newline
                    self.cursor.line_offset += 1;
                    self.cursor.original_line_offset = self.cursor.line_offset;
                }
            }
        } else {
            // cursor should be still within the same node
            //	increase the node_offset by 1
            //	update the line index based on whether it has crossed
            //  over a newline
            self.cursor.node_offset += 1;
            let current_node = &self.node_list.get_curr();
            if self.cursor.line_idx + 1 <= current_node.line_offsets_len() - 1
                && self.cursor.node_offset == current_node.line_offset_at(self.cursor.line_idx + 1)
            {
                // cursor should have just passed over a '\n'
                self.current_line += 1;
                self.cursor.line_idx += 1;
                self.cursor.line_offset = 0;
                self.cursor.original_line_offset = self.cursor.line_offset;
            } else {
                self.cursor.line_offset += 1;
                self.cursor.original_line_offset = self.cursor.line_offset;
            }
        }
    }
}

#[cfg(test)]
#[path = "tests/buffer_tests.rs"]
mod buffer_tests;
