#[derive(Debug)]
pub struct Cursor {
    //pub node_idx: usize,    // current node to reference
    pub node_offset: usize, // the offset based on the current node
    pub line_idx: usize,    // current line based on the node
    pub line_offset: usize, // actual/current offset from left newline
    pub original_line_offset: usize, // original offset from left newline
                            // nota bene: original_line_offset should only be used for informing the cursor what
                            //  should be when it is being moved in between lines
                            //  any edits or left/right movement of the cursor should reset this value
                            //  to what the current line_offset should be
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            //node_idx: 0,
            node_offset: 0,
            line_idx: 0,
            line_offset: 0,
            original_line_offset: 0,
        }
    }
}
