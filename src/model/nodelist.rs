use std::collections::VecDeque;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BufferType {
    Original,
    Added,
}

#[derive(Clone, PartialEq, Debug)]
pub struct BufferNode {
    from: BufferType,
    index: usize,
    offset: usize,
    line_offsets: Vec<usize>,
}

impl BufferNode {
    pub fn new(
        from: BufferType,
        index: usize,
        offset: usize,
        line_offsets: Vec<usize>,
    ) -> BufferNode {
        BufferNode {
            from,
            index,
            offset,
            line_offsets,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn from(&self) -> BufferType {
        self.from
    }

    pub fn line_offsets(&mut self) -> &mut Vec<usize> {
        &mut self.line_offsets
    }

    pub fn line_offset_at(&self, idx: usize) -> usize {
        self.line_offsets[idx]
    }

    pub fn line_offsets_len(&self) -> usize {
        self.line_offsets.len()
    }

    pub fn last_line_offset(&self) -> usize {
        *self.line_offsets.last().unwrap()
    }

    pub fn has_newline(&self) -> bool {
        self.line_offsets_len() > 1
    }
}

#[derive(Debug)]
pub struct NodeList {
    pub left_list: VecDeque<BufferNode>,
    pub right_list: VecDeque<BufferNode>,
}

#[allow(dead_code)]
impl NodeList {
    pub fn new() -> NodeList {
        NodeList {
            left_list: VecDeque::new(),
            right_list: VecDeque::new(),
        }
    }

    pub fn with_contents(mut contents: Vec<BufferNode>) -> NodeList {
        let left_list: VecDeque<_> = contents.drain(..=0).collect();
        let right_list: VecDeque<_> = contents.drain(..).collect();
        NodeList {
            left_list: left_list,
            right_list: right_list,
        }
    }

    pub fn is_empty(&self) -> bool {
        assert!(!self.left_list.is_empty() || self.right_list.is_empty());
        self.left_list.is_empty()
    }

    pub fn len(&self) -> usize {
        self.left_list.len() + self.right_list.len()
    }

    pub fn index(&self) -> usize {
        assert!(!self.left_list.is_empty());
        self.left_list.len() - 1
    }

    pub fn shift_to_index(&mut self, index: usize) {
        assert!(index < self.len());
        while self.index() < index {
            self.move_right();
        }

        while self.index() > index {
            self.move_left();
        }
    }

    pub fn get_curr(&self) -> &BufferNode {
        self.left_list.back().unwrap()
    }

    pub fn get_next(&self) -> &BufferNode {
        self.right_list.front().unwrap()
    }

    pub fn get_prev(&self) -> &BufferNode {
        assert!(self.left_list.len() >= 2);
        self.left_list.get(self.left_list.len() - 2).unwrap()
    }

    pub fn get_curr_mut(&mut self) -> &mut BufferNode {
        self.left_list.back_mut().unwrap()
    }

    pub fn get_next_mut(&mut self) -> &mut BufferNode {
        self.right_list.front_mut().unwrap()
    }

    pub fn get_prev_mut(&mut self) -> &mut BufferNode {
        assert!(self.left_list.len() >= 2);
        let idx = self.left_list.len() - 2;
        self.left_list.get_mut(idx).unwrap()
    }

    pub fn insert_curr(&mut self, node: BufferNode) {
        self.left_list.push_back(node);
    }

    pub fn insert_next(&mut self, node: BufferNode) {
        if self.left_list.is_empty() {
            self.left_list.push_front(node);
        } else {
            self.right_list.push_front(node);
        }
    }

    pub fn insert_prev(&mut self, node: BufferNode) {
        if self.left_list.is_empty() {
            self.left_list.push_front(node);
            return;
        }
        let idx = self.left_list.len() - 1;
        self.left_list.insert(idx, node);
    }

    pub fn remove_curr(&mut self) {
        self.left_list.pop_back();
        if self.left_list.is_empty() && !self.right_list.is_empty() {
            let element = self.right_list.pop_front().unwrap();
            self.left_list.push_back(element);
        }
    }

    pub fn remove_prev(&mut self) {
        if self.left_list.len() < 2 {
            return;
        }

        let element = self.left_list.pop_back().unwrap();
        self.left_list.pop_back();
        self.left_list.push_back(element);
    }

    pub fn remove_next(&mut self) {
        self.right_list.pop_front();
    }

    pub fn at_head(&self) -> bool {
        return self.left_list.len() == 1;
    }

    pub fn at_tail(&self) -> bool {
        return self.right_list.len() == 0 && !self.is_empty();
    }

    pub fn move_left(&mut self) {
        if self.left_list.len() < 2 {
            return;
        }

        let element = self.left_list.pop_back().unwrap();
        self.right_list.push_front(element);
    }

    pub fn move_right(&mut self) {
        if self.right_list.len() < 1 {
            return;
        }

        let element = self.right_list.pop_front().unwrap();
        self.left_list.push_back(element);
    }

    pub fn get(&self, index: usize) -> &BufferNode {
        assert!(!self.is_empty());
        if index < self.left_list.len() {
            self.left_list.get(index).unwrap()
        } else if index < self.len() {
            let index = index - self.left_list.len();
            self.right_list.get(index).unwrap()
        } else {
            panic!("Index out of bounds! or something...");
        }
    }

    pub fn get_mut(&mut self, index: usize) -> &BufferNode {
        assert!(!self.is_empty());
        if index < self.left_list.len() {
            self.left_list.get_mut(index).unwrap()
        } else if index < self.len() {
            let index = index - self.left_list.len();
            self.right_list.get_mut(index).unwrap()
        } else {
            panic!("Index out of bounds! or something...");
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &BufferNode> {
        self.left_list.iter().chain(self.right_list.iter())
    }

    pub fn iter_until_curr(&self) -> impl Iterator<Item = &BufferNode> {
        self.left_list.iter()
    }

    pub fn iter_from_after_curr(&self) -> impl Iterator<Item = &BufferNode> {
        self.right_list.iter()
    }

    pub fn move_to_prev_newline(&mut self) {
        self.move_left();
        while !self.get_curr().has_newline() && !self.at_head() {
            self.move_left();
        }
    }

    pub fn move_to_next_newline(&mut self) {
        self.move_right();
        while !self.get_curr().has_newline() && !self.at_tail() {
            self.move_right();
        }
    }
}

impl PartialEq<Vec<BufferNode>> for NodeList {
    fn eq(&self, other: &Vec<BufferNode>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for idx in 0..self.len() {
            if self.get(idx) == other.get(idx).unwrap() {
                continue;
            }
            return false;
        }
        true
    }
}

#[cfg(test)]
#[path = "tests/nodelist_tests.rs"]
mod node_list_tests;
