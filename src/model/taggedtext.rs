use super::texttag::TextTag;

pub struct TaggedText {
    text: String,
    tags: Vec<TextTag>,
}

impl TaggedText {
    pub fn new(text: String, mut tags: Vec<TextTag>) -> TaggedText {
        tags.sort_unstable();
        TaggedText{
            text,
            tags,
        }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn tags(&self) -> &Vec<TextTag> {
        &self.tags
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }

    pub fn tags_mut(&mut self) -> &mut Vec<TextTag> {
        &mut self.tags
    }
}