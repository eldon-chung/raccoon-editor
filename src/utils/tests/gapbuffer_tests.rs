#[cfg(test)]
mod gapbuffer_tests {
    use super::super::*;

    #[test]
    fn test_insert_after_on_empty() {
        let mut gap_buffer = GapBuffer::new();

        gap_buffer.insert_after(0);

        assert_eq!(gap_buffer.left_list, [0]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn test_insert_after_on_non_empty() {
        let mut gap_buffer = GapBuffer::new();

        gap_buffer.insert_after(0);
        gap_buffer.insert_after(1);

        assert_eq!(gap_buffer.left_list, [0]);
        assert_eq!(gap_buffer.right_list, [1]);
    }

    #[test]
    fn test_insert_twice_after_on_non_empty() {
        let mut gap_buffer = GapBuffer::new();

        gap_buffer.insert_after(0);
        gap_buffer.insert_after(1);
        gap_buffer.insert_after(2);

        assert_eq!(gap_buffer.left_list, [0]);
        assert_eq!(gap_buffer.right_list, [2, 1]);
    }

    #[test]
    fn test_insert_before_on_empty() {
        let mut gap_buffer = GapBuffer::new();
        gap_buffer.insert_before(0);

        assert_eq!(gap_buffer.left_list, [0]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn test_insert_before_on_non_empty() {
        let mut gap_buffer = GapBuffer::new();

        gap_buffer.insert_before(0);
        gap_buffer.insert_before(1);

        assert_eq!(gap_buffer.left_list, [1, 0]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn test_insert_twice_before_on_non_empty() {
        let mut gap_buffer = GapBuffer::new();

        gap_buffer.insert_before(0);
        gap_buffer.insert_before(1);
        gap_buffer.insert_before(2);

        assert_eq!(gap_buffer.left_list, [1, 2, 0]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn delete_before_on_empty() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();

        gap_buffer.delete_before();

        assert!(gap_buffer.left_list.is_empty());
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn delete_before_on_left() {
        let mut gap_buffer = GapBuffer::new();

        gap_buffer.insert_after(0);
        gap_buffer.delete_before();

        assert_eq!(gap_buffer.left_list, [0]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn delete_before_on_left_2() {
        let mut gap_buffer = GapBuffer::new();

        gap_buffer.insert_after(0);
        gap_buffer.insert_after(1);
        gap_buffer.delete_before();

        assert_eq!(gap_buffer.left_list, [0]);
        assert_eq!(gap_buffer.right_list, [1]);
    }

    #[test]
    fn delete_before_on_left_right() {
        let mut gap_buffer = GapBuffer::new();

        gap_buffer.insert_after(0);
        gap_buffer.insert_after(1);
        gap_buffer.right_list.push_back(2);
        gap_buffer.delete_before();

        assert_eq!(gap_buffer.left_list, [0]);
        assert_eq!(gap_buffer.right_list, [1, 2]);
    }

    #[test]
    fn delete_after_on_empty() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();

        gap_buffer.delete_after();

        assert!(gap_buffer.left_list.is_empty());
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn delete_after_on_empty_right() {
        let mut gap_buffer = GapBuffer::new();

        gap_buffer.insert_after(0);
        gap_buffer.delete_after();

        assert_eq!(gap_buffer.left_list, [0]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn delete_after_on_right() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();

        gap_buffer.insert_after(0);
        gap_buffer.delete_after();

        assert_eq!(gap_buffer.left_list, [0]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn delete_after_on_left_2() {
        let mut gap_buffer = GapBuffer::new();

        gap_buffer.insert_after(0);
        gap_buffer.insert_after(1);
        gap_buffer.delete_after();

        assert_eq!(gap_buffer.left_list, [0]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn delete_after_on_left_right() {
        let mut gap_buffer = GapBuffer::new();

        gap_buffer.insert_after(0);
        gap_buffer.insert_after(1);
        gap_buffer.right_list.push_back(2);
        gap_buffer.delete_after();

        assert_eq!(gap_buffer.left_list, [0]);
        assert_eq!(gap_buffer.right_list, [2]);
    }

    #[test]
    fn delete_current_on_empty() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.delete_current();

        assert!(gap_buffer.left_list.is_empty());
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn delete_current_on_left() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.insert_after(0);
        gap_buffer.delete_current();

        assert!(gap_buffer.left_list.is_empty());
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn delete_current_on_left_2() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.insert_after(1);
        gap_buffer.insert_before(0);
        gap_buffer.delete_current();

        assert_eq!(gap_buffer.left_list, [0]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn delete_current_on_left_right() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.insert_after(1);
        gap_buffer.insert_before(0);
        gap_buffer.insert_after(2);
        gap_buffer.delete_current();

        assert_eq!(gap_buffer.left_list, [0]);
        assert_eq!(gap_buffer.right_list, [2]);
    }

    #[test]
    fn delete_current_on_left_right_2() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.insert_before(0);
        gap_buffer.insert_after(2);
        gap_buffer.insert_after(1);
        gap_buffer.delete_current();

        assert_eq!(gap_buffer.left_list, [1]);
        assert_eq!(gap_buffer.right_list, [2]);
    }

    #[test]
    fn delete_current_twice_on_left_right() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.insert_after(1);
        gap_buffer.insert_before(0);
        gap_buffer.insert_after(2);
        gap_buffer.delete_current();
        gap_buffer.delete_current();

        assert_eq!(gap_buffer.left_list, [2]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn delete_current_left_size_2() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.left_list.push_back(0);
        gap_buffer.left_list.push_back(1);
        gap_buffer.delete_current();

        assert_eq!(gap_buffer.left_list, [0]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn delete_current_thrice_on_left_right() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.insert_after(1);
        gap_buffer.insert_before(0);
        gap_buffer.insert_after(2);
        gap_buffer.delete_current();
        gap_buffer.delete_current();
        gap_buffer.delete_current();

        assert!(gap_buffer.left_list.is_empty());
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn move_pointer_right_on_empty() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.move_pointer_right();

        assert!(gap_buffer.left_list.is_empty());
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn move_pointer_right_on_left() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.insert_after(0);
        gap_buffer.move_pointer_right();

        assert_eq!(gap_buffer.left_list, [0]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn move_pointer_right_on_left_right() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.insert_after(0);
        gap_buffer.insert_after(1);
        gap_buffer.move_pointer_right();

        assert_eq!(gap_buffer.left_list, [0, 1]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn move_pointer_left_on_empty() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.move_pointer_left();

        assert!(gap_buffer.left_list.is_empty());
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn move_pointer_left_on_left() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.insert_before(0);
        gap_buffer.move_pointer_left();

        assert_eq!(gap_buffer.left_list, [0]);
        assert!(gap_buffer.right_list.is_empty());
    }

    #[test]
    fn move_pointer_left_on_left_size_2() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.insert_before(0);
        gap_buffer.left_list.push_back(1);
        gap_buffer.move_pointer_left();

        assert_eq!(gap_buffer.left_list, [0]);
        assert_eq!(gap_buffer.right_list, [1]);
    }

    #[test]
    fn move_pointer_left_on_left_right_2() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.insert_before(0);
        gap_buffer.insert_after(1);
        gap_buffer.move_pointer_left();

        assert_eq!(gap_buffer.left_list, [0]);
        assert_eq!(gap_buffer.right_list, [1]);
    }

    #[test]
    fn move_pointer_left_on_left_right() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        gap_buffer.insert_before(1);
        gap_buffer.left_list.push_front(0);
        gap_buffer.move_pointer_left();

        assert_eq!(gap_buffer.left_list, [0]);
        assert_eq!(gap_buffer.right_list, [1]);
    }

    #[test]
    fn get_on_empty() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();

        assert!(gap_buffer.get_prev().is_none());
        assert!(gap_buffer.get_current().is_none());
        assert!(gap_buffer.get_next().is_none());
    }

    #[test]
    fn get_current() {
        let mut gap_buffer = GapBuffer::with_contents(vec![0]);

        assert!(gap_buffer.get_next().is_none());
        assert_eq!(*gap_buffer.get_current().unwrap(), 0);
        assert!(gap_buffer.get_next().is_none());
    }

    #[test]
    fn get_current_and_left() {
        let mut gap_buffer = GapBuffer::with_contents(vec![0, 1]);

        assert_eq!(*gap_buffer.get_prev().unwrap(), 0);
        assert_eq!(*gap_buffer.get_current().unwrap(), 1);
        assert!(gap_buffer.get_next().is_none());
    }

    #[test]
    fn get_current_and_left_and_right() {
        let mut gap_buffer = GapBuffer::with_contents(vec![0, 1, 2]);

        gap_buffer.move_pointer_left();

        assert_eq!(*gap_buffer.get_prev().unwrap(), 0);
        assert_eq!(*gap_buffer.get_current().unwrap(), 1);
        assert_eq!(*gap_buffer.get_next().unwrap(), 2);
    }

    #[test]
    fn get_current_and_right() {
        let mut gap_buffer = GapBuffer::with_contents(vec![0, 1]);

        gap_buffer.move_pointer_left();

        assert!(gap_buffer.get_prev().is_none());
        assert_eq!(*gap_buffer.get_current().unwrap(), 0);
        assert_eq!(*gap_buffer.get_next().unwrap(), 1);
    }

    #[test]
    fn get_len() {
        let mut gap_buffer = GapBuffer::new();
        gap_buffer.insert_after(0);
        gap_buffer.insert_after(1);
        assert_eq!(gap_buffer.len(), 2);
    }

    #[test]
    fn get_idx() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        assert_eq!(gap_buffer.len(), 0);
    }

    #[test]
    fn is_head() {
        let mut gap_buffer = GapBuffer::new();
        gap_buffer.insert_after(0);
        assert!(gap_buffer.is_head());
    }

    #[test]
    fn is_tail() {
        let mut gap_buffer = GapBuffer::new();
        gap_buffer.insert_after(0);
        assert!(gap_buffer.is_tail());
    }

    #[test]
    fn is_empty() {
        let mut gap_buffer: GapBuffer<u32> = GapBuffer::new();
        assert!(gap_buffer.is_empty());
    }
}