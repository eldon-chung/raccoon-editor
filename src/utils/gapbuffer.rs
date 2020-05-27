use std::collections::VecDeque;
use std::iter::Iterator;

#[derive(Debug)]
pub struct GapBuffer<E> {
    pub left_list: VecDeque<E>,
    pub right_list: VecDeque<E>,
}

#[allow(dead_code)]
impl<E> GapBuffer<E> {
    pub fn new() -> GapBuffer<E> {
        GapBuffer {
            left_list: VecDeque::new(),
            right_list: VecDeque::new(),
        }
    }

    pub fn with_contents(mut contents: Vec<E>) -> GapBuffer<E> {
        let left_list: VecDeque<E> = contents.drain(..).collect();
        GapBuffer {
            left_list: left_list,
            right_list: VecDeque::new(),
        }
    }

    pub fn insert_after(&mut self, element: E) {
        if self.left_list.is_empty() {
            self.left_list.push_front(element);
            return;
        }
        self.right_list.push_front(element);
    }

    pub fn insert_before(&mut self, element: E) {
        if self.left_list.is_empty() {
            self.left_list.push_front(element);
            return;
        }
        let current_element = self.left_list.pop_back();
        if current_element.is_none() {
            return;
        }
        self.left_list.push_back(element);
        self.left_list.push_back(current_element.unwrap());
    }

    pub fn delete_after(&mut self) {
        self.right_list.pop_front();
    }

    pub fn delete_before(&mut self) {
        let current_element = self.left_list.pop_back();
        self.left_list.pop_back();
        if current_element.is_none() {
            return;
        }
        self.left_list.push_back(current_element.unwrap());
    }

    pub fn delete_current(&mut self) {
        self.left_list.pop_back();
        if !self.left_list.is_empty() {
            return;
        }
        let current_element = self.right_list.pop_front();
        if current_element.is_none() {
            return;
        }
        self.left_list.push_back(current_element.unwrap());
    }

    pub fn move_pointer_right(&mut self) {
        let current_element = self.right_list.pop_front();
        if current_element.is_none() {
            return;
        }
        self.left_list.push_back(current_element.unwrap());
    }

    pub fn move_pointer_left(&mut self) {
        if self.left_list.len() <= 1 {
            return;
        }
        let current_element = self.left_list.pop_back();
        self.right_list.push_front(current_element.unwrap());
    }

    pub fn get_current(&mut self) -> Option<&E> {
        self.left_list.back()
    }

    pub fn get_next(&mut self) -> Option<&E> {
        self.right_list.front()
    }

    pub fn get_prev(&mut self) -> Option<&E> {
        if self.left_list.len() < 2 {
            return None;
        }
        self.left_list.get(self.left_list.len() - 2)
    }

    pub fn get_current_mut(&mut self) -> Option<&mut E> {
        self.left_list.back_mut()
    }

    pub fn get_next_mut(&mut self) -> Option<&mut E> {
        self.right_list.front_mut()
    }

    pub fn get_prev_mut(&mut self) -> Option<&mut E> {
        if self.left_list.len() < 2 {
            return None;
        }

        self.left_list.get_mut(self.left_list.len() - 2)
    }

    pub fn get_idx(&self) -> usize {
        self.left_list.len() - 1
    }

    pub fn len(&self) -> usize {
        self.left_list.len() + self.right_list.len()
    }

    pub fn is_head(&self) -> bool {
        self.left_list.len() == 1
    }

    pub fn is_tail(&self) -> bool {
        return self.right_list.is_empty() && !self.left_list.is_empty();
    }

    pub fn is_empty(&self) -> bool {
        assert!(!(self.left_list.is_empty()) || (self.right_list.is_empty()));
        self.left_list.is_empty()
    }

    pub fn left_list_as_vec(&self) -> Vec<&E> {
        let left_list_copy: Vec<_> = self.left_list.iter().collect();
        left_list_copy
    }

    pub fn right_list_as_vec(&self) -> Vec<&E> {
        let right_list_copy: Vec<_> = self.right_list.iter().collect();
        right_list_copy
    }

    pub fn left_right_list_as_vec(&self) -> Vec<&E> {
        let mut left_list_copy: Vec<_> = self.left_list.iter().collect();
        let mut right_list_copy: Vec<_> = self.right_list.iter().collect();
        left_list_copy.append(&mut right_list_copy);
        left_list_copy
    }

    #[cfg(test)]
    pub fn left_list(&mut self) -> Vec<E> {
        self.left_list.drain(..).collect()
    }

    #[cfg(test)]
    pub fn right_list(&mut self) -> Vec<E> {
        self.right_list.drain(..).collect()
    }
}

#[cfg(test)]
#[path = "tests/gapbuffer_tests.rs"]
mod gapbuffer_tests;