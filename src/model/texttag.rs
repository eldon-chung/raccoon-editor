use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
pub struct TextTag {
    tag: Tag,
    start_idx: usize,
    end_idx: usize,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Tag {
    Cursor,
    Highlighted,
}

impl TextTag {
    pub fn new(tag: Tag, start_idx: usize, end_idx: usize) -> TextTag {
        TextTag {
            tag,
            start_idx, // inclusive
            end_idx,   // exclusive
        }
    }

    pub fn tag(&self) -> Tag {
        self.tag
    }

    pub fn start_idx(&self) -> usize {
        self.start_idx
    }

    pub fn end_idx(&self) -> usize {
        self.end_idx
    }
}

impl PartialOrd for TextTag {
    fn partial_cmp(&self, other: &TextTag) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TextTag {
    fn cmp(&self, other: &TextTag) -> Ordering {
        if self.start_idx() == other.start_idx() {
            self.end_idx().cmp(&other.end_idx())
        } else {
            self.start_idx.cmp(&other.start_idx())
        }
    }
}
