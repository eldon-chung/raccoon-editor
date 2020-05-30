#[cfg(test)]
mod node_list_tests {
    use super::super::*;

    #[test]
    fn is_empty_on_new_list() {
        let node_list = NodeList::new();
        assert!(node_list.is_empty());
    }

    #[test]
    #[should_panic]
    fn is_empty_panic_on_list() {
        let mut node_list = NodeList::new();
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        node_list.right_list.push_front(node_0);
        node_list.is_empty();
    }

    #[test]
    fn len_on_new_list() {
        let mut node_list = NodeList::new();
        assert_eq!(node_list.len(), 0);
    }

    #[test]
    fn len() {
        let mut node_list = NodeList::new();
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_2 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        node_list.right_list.push_front(node_0);
        node_list.right_list.push_front(node_1);
        node_list.left_list.push_front(node_2);
        assert_eq!(node_list.len(), 3);
    }

    #[test]
    fn shift_to_index() {
        let node_0 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 2, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 2, 0, vec![0]);
        let node_2 = BufferNode::new(BufferType::Original, 3, 0, vec![0]);
        let node_2_copy = BufferNode::new(BufferType::Original, 3, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1, node_2]);
        assert_eq!(*node_list.get_curr(), node_0_copy);

        node_list.shift_to_index(2);
        assert_eq!(*node_list.get_curr(), node_2_copy);

        node_list.shift_to_index(1);
        assert_eq!(*node_list.get_curr(), node_1_copy);

        node_list.shift_to_index(0);
        assert_eq!(*node_list.get_curr(), node_0_copy);
    }

    #[test]
    #[should_panic]
    fn shift_to_index_panic() {
        let node_0 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 2, 0, vec![0]);
        let node_2 = BufferNode::new(BufferType::Original, 3, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1, node_2]);
        node_list.shift_to_index(3);
    }

    #[test]
    #[should_panic]
    fn get_curr_on_empty() {
        let node_list = NodeList::new();
        node_list.get_curr();
    }

    #[test]
    fn get_curr() {
        let node_0 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_list = NodeList::with_contents(vec![node_0]);
        assert_eq!(*node_list.get_curr(), node_0_copy);
    }

    #[test]
    #[should_panic]
    fn get_next_without_next() {
        let node_0 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_list = NodeList::with_contents(vec![node_0]);
        node_list.get_next();
    }

    #[test]
    #[should_panic]
    fn get_prev_without_prev() {
        let node_0 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_list = NodeList::with_contents(vec![node_0]);
        node_list.get_prev();
    }

    #[test]
    #[should_panic]
    fn get_next_mut_without_next() {
        let node_0 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0]);
        node_list.get_next_mut();
    }

    #[test]
    #[should_panic]
    fn get_prev_mut_without_prev() {
        let node_0 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0]);
        node_list.get_prev_mut();
    }

    #[test]
    #[should_panic]
    fn get_curr_mut_on_empty() {
        let mut node_list = NodeList::new();
        node_list.get_curr_mut();
    }

    #[test]
    fn insert_curr_into_empty() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let mut node_list = NodeList::new();
        node_list.insert_curr(node_0);
        assert_eq!(node_list.left_list, [node_0_copy]);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn insert_curr_into_non_empty_1() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0]);
        node_list.insert_curr(node_1);
        assert_eq!(node_list.left_list, [node_0_copy, node_1_copy]);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn insert_curr_into_non_empty_2() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_2 = BufferNode::new(BufferType::Original, 2, 0, vec![0]);
        let node_2_copy = BufferNode::new(BufferType::Original, 2, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        node_list.move_left();
        node_list.insert_curr(node_2);
        assert_eq!(node_list.left_list, [node_0_copy, node_2_copy]);
        assert_eq!(node_list.right_list, [node_1_copy]);
    }

    #[test]
    fn insert_next_into_empty() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let mut node_list = NodeList::new();
        node_list.insert_next(node_0);
        assert_eq!(node_list.left_list, [node_0_copy]);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn insert_next_into_non_empty_1() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0]);
        node_list.insert_next(node_1);
        assert_eq!(node_list.left_list, [node_0_copy]);
        assert_eq!(node_list.right_list, [node_1_copy]);
    }

    #[test]
    fn insert_next_into_non_empty_2() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_2 = BufferNode::new(BufferType::Original, 2, 0, vec![0]);
        let node_2_copy = BufferNode::new(BufferType::Original, 2, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        node_list.move_left();
        node_list.insert_next(node_2);
        assert_eq!(node_list.left_list, [node_0_copy]);
        assert_eq!(node_list.right_list, [node_2_copy, node_1_copy]);
    }

    #[test]
    fn insert_prev_into_empty() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let mut node_list = NodeList::new();
        node_list.insert_prev(node_0);
        assert_eq!(node_list.left_list, [node_0_copy]);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn insert_prev_into_non_empty_1() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0]);
        node_list.insert_prev(node_1);
        assert_eq!(node_list.left_list, [node_1_copy, node_0_copy]);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn insert_prev_into_non_empty_2() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_2 = BufferNode::new(BufferType::Original, 2, 0, vec![0]);
        let node_2_copy = BufferNode::new(BufferType::Original, 2, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        node_list.move_left();
        node_list.insert_prev(node_2);
        assert_eq!(node_list.left_list, [node_2_copy, node_0_copy]);
        assert_eq!(node_list.right_list, [node_1_copy]);
    }

    #[test]
    fn remove_curr_from_non_empty_1() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0]);
        node_list.remove_curr();
        assert_eq!(node_list.left_list, []);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn remove_curr_from_non_empty_2() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        node_list.move_right();
        node_list.remove_curr();
        assert_eq!(node_list.left_list, [node_0_copy]);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn remove_curr_from_non_empty_3() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        node_list.move_left();
        node_list.remove_curr();
        assert_eq!(node_list.left_list, [node_1_copy]);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn remove_prev_from_non_empty_1() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0]);
        node_list.remove_prev();
        assert_eq!(node_list.left_list, [node_0_copy]);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn remove_prev_from_non_empty_2() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        node_list.move_right();
        node_list.remove_prev();
        assert_eq!(node_list.left_list, [node_1_copy]);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn remove_prev_from_non_empty_3() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        node_list.move_left();
        node_list.remove_prev();
        assert_eq!(node_list.left_list, [node_0_copy]);
        assert_eq!(node_list.right_list, [node_1_copy]);
    }

    #[test]
    fn remove_next_from_non_empty_1() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0]);
        node_list.remove_next();
        assert_eq!(node_list.left_list, [node_0_copy]);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn remove_next_from_non_empty_2() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        node_list.move_right();
        node_list.remove_next();
        assert_eq!(node_list.left_list, [node_0_copy, node_1_copy]);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn remove_next_from_non_empty_3() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        node_list.remove_next();
        assert_eq!(node_list.left_list, [node_0_copy]);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn at_head_on_empty() {
        let node_list = NodeList::new();
        assert!(!node_list.at_head());
    }

    #[test]
    fn at_tail_on_empty() {
        let node_list = NodeList::new();
        assert!(!node_list.at_tail());
    }

    #[test]
    fn at_head() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_list = NodeList::with_contents(vec![node_0]);
        assert!(node_list.at_head());
    }

    #[test]
    fn at_tail() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_list = NodeList::with_contents(vec![node_0]);
        assert!(node_list.at_tail());
    }

    #[test]
    fn move_left_at_head() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        node_list.move_left();
        assert_eq!(node_list.left_list, [node_0_copy]);
        assert_eq!(node_list.right_list, [node_1_copy]);
    }

    #[test]
    fn move_right_at_tail() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        node_list.move_right();
        node_list.move_right();
        assert_eq!(node_list.left_list, [node_0_copy, node_1_copy]);
        assert_eq!(node_list.right_list, []);
    }

    #[test]
    fn move_right_left() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        node_list.move_right();
        node_list.move_left();
        assert_eq!(node_list.left_list, [node_0_copy]);
        assert_eq!(node_list.right_list, [node_1_copy]);
    }

    #[test]
    #[should_panic]
    fn get_on_empty() {
        let node_list = NodeList::new();
        node_list.get(0);
    }

    #[test]
    #[should_panic]
    fn get_with_larger_idx() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        let idx = node_list.len();
        node_list.get(idx);
    }

    #[test]
    fn get_on_nonempty_list() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        let idx = node_list.len();
        assert_eq!(*node_list.get(0), node_0_copy);
    }

    #[test]
    #[should_panic]
    fn get_mut_on_empty() {
        let mut node_list = NodeList::new();
        node_list.get_mut(0);
    }

    #[test]
    #[should_panic]
    fn get_mut_with_larger_idx() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        let idx = node_list.len();
        node_list.get_mut(idx);
    }

    #[test]
    fn get_mut_on_nonempty_list() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let mut node_list = NodeList::with_contents(vec![node_0, node_1]);
        let idx = node_list.len();
        assert_eq!(*node_list.get_mut(0), node_0_copy);
    }

    #[test]
    fn get_iter_on_empty_list() {
        let node_list = NodeList::new();
        let mut iter = node_list.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn get_iter_on_nonempty_list() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_list = NodeList::with_contents(vec![node_0, node_1]);
        let mut iter = node_list.iter();
        assert_eq!(*iter.next().unwrap(), node_0_copy);
        assert_eq!(*iter.next().unwrap(), node_1_copy);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn get_iter_until_curr_on_nonempty_list() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_list = NodeList::with_contents(vec![node_0, node_1]);
        let mut iter = node_list.iter_until_curr();
        assert_eq!(*iter.next().unwrap(), node_0_copy);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn get_iter_from_after_curr_on_nonempty_list() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_list = NodeList::with_contents(vec![node_0, node_1]);
        let mut iter = node_list.iter_from_after_curr();
        assert_eq!(*iter.next().unwrap(), node_1_copy);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn eq() {
        let node_0 = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_0_copy = BufferNode::new(BufferType::Original, 0, 0, vec![0]);
        let node_1 = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_1_copy = BufferNode::new(BufferType::Original, 1, 0, vec![0]);
        let node_list = NodeList::with_contents(vec![node_0, node_1]);
        assert_eq!(node_list, vec![node_0_copy, node_1_copy]);
    }
}
