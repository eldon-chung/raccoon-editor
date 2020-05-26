use std::collections::VecDeque;

#[derive(Debug)]
pub struct GapBuffer<E> {
    left_list: VecDeque<E>,
    right_list: VecDeque<E>,
}

impl <E>GapBuffer<E> {
    fn new() -> GapBuffer<E> {
        GapBuffer {
            left_list: VecDeque::new(),
            right_list: VecDeque::new(),
        }
    }

    fn with_contents(mut contents: Vec<E>) -> GapBuffer<E> {
        let left_list: VecDeque<E> = contents.drain(..).collect();
        GapBuffer {
            left_list: left_list,
            right_list: VecDeque::new(),
        }
    }

    fn insert_after(&mut self, element: E) {
        if self.left_list.is_empty() {
            self.left_list.push_front(element);
            return;
        }
        self.right_list.push_front(element);
    }

    fn insert_before(&mut self, element: E) {
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

    fn delete_after(&mut self) {
        self.right_list.pop_front();
    }

    fn delete_before(&mut self) {
        let current_element = self.left_list.pop_back();
        self.left_list.pop_back();
        if current_element.is_none() {
            return;
        }
        self.left_list.push_back(current_element.unwrap());
    }

    fn delete_current(&mut self) {
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

    fn move_pointer_right(&mut self) {
        let current_element = self.right_list.pop_front();
        if current_element.is_none() {
            return;
        }
        self.left_list.push_back(current_element.unwrap());
    }

    fn move_pointer_left(&mut self) {
        if self.left_list.len() <= 1 {
            return;
        }
        let current_element = self.left_list.pop_back();
        self.right_list.push_front(current_element.unwrap());
    }

    fn get_current(&mut self) -> Option<&E> {
        self.left_list.back()
    }

    fn get_next(&mut self) -> Option<&E> {
        self.right_list.front()
    }

    fn get_prev(&mut self) -> Option<&E> {
        if self.left_list.len() < 2 {
            return None;
        }
        self.left_list.get(self.left_list.len() - 2)
    }

    fn get_current_mut(&mut self) -> Option<&mut E> {
        self.left_list.back_mut()
    }

    fn get_next_mut(&mut self) -> Option<&mut E> {
        self.right_list.front_mut()
    }

    fn get_prev_mut(&mut self) -> Option<&mut E> {
        if self.left_list.len() < 2 {
            return None;
        }

        self.left_list.get_mut(self.left_list.len() - 2)
    }

    fn get_idx(&self) -> usize {
        self.left_list.len() - 1
    }

    fn len(&self) -> usize {
        self.left_list.len() + self.right_list.len()
    }

    fn is_head(&self) -> bool {
        self.left_list.len() == 1
    }

    fn is_tail(&self) -> bool {
        return self.right_list.is_empty()
            && !self.left_list.is_empty();
    }

    fn is_empty(&self) -> bool {
        assert!(!(self.left_list.len() == 0) || (self.right_list.len() == 0));
        self.left_list.is_empty()
    }
}

#[cfg(test)]
#[path = "tests/gapbuffer_tests.rs"]
mod gapbuffer_tests;
