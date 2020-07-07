use super::texttag::TextTag;

macro_rules! min {
    ($x:expr, $y:expr) => {
        if $x < $y {
            $x
        } else {
            $y
        }
    };
}

macro_rules! max {
    ($x:expr, $y:expr) => {
        if $x > $y {
            $x
        } else {
            $y
        }
    };
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaggedText {
    text: String,
    tags: Vec<TextTag>,
}

impl TaggedText {
    pub fn new(text: String, mut tags: Vec<TextTag>) -> TaggedText {
        tags.sort_unstable();
        TaggedText { text, tags }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }

    pub fn tags(&self) -> &Vec<TextTag> {
        &self.tags
    }

    pub fn push_tag(&mut self, texttag: TextTag) {
        let idx_result = self.tags.binary_search(&texttag);
        match idx_result {
            Ok(idx) => self.tags.insert(idx, texttag),
            Err(idx) => self.tags.insert(idx, texttag),
        };
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }

    pub fn tags_mut(&mut self) -> &mut Vec<TextTag> {
        &mut self.tags
    }

    /**
     * Split TaggedText by whitespace character.
     *
     * @return to_return
     */
    pub fn split_whitespace(self) -> Vec<TaggedText> {
        if self.text() == "" {
            return vec![self];
        }

        let mut to_return: Vec<TaggedText> = Vec::new();
        let text = self.text();

        let mut cumulative_lengths = Vec::new(); // cumulative word lengths
        cumulative_lengths.push(0);
        for word in text.split(" ") {
            to_return.push(TaggedText::new(word.to_string(), Vec::new()));
            let last_length = cumulative_lengths.last().unwrap();
            cumulative_lengths.push(last_length + word.len() + 1);
        }

        let tags = self.tags();
        for tag in tags {
            let left_idx = match cumulative_lengths.binary_search(&tag.start_idx()) {
                Ok(idx) => idx,
                Err(idx) => idx - 1,
            };

            let right_idx = match cumulative_lengths.binary_search(&tag.end_idx()) {
                Ok(idx) => idx,
                Err(idx) => idx,
            };

            for idx in left_idx..right_idx {
                let start = max!(tag.start_idx(), cumulative_lengths[idx]);
                let end = min!(tag.end_idx(), cumulative_lengths[idx + 1] - 1);

                let text_tag = TextTag::new(
                    tag.tag(),
                    start - cumulative_lengths[idx],
                    end - cumulative_lengths[idx],
                );
                to_return[idx].push_tag(text_tag);
            }
        }
        to_return
    }

    /**
     * Join multiple TaggedText into a single TaggedText by a specifited delimiter character
     *
     * @param tagged_texts
     * @param delimiter
     */
    pub fn join(mut tagged_texts: Vec<TaggedText>, delimiter: char) -> TaggedText {
        if tagged_texts.len() == 1 {
            return tagged_texts.remove(0);
        }

        let mut joined_text = String::new();
        let mut joined_tags = Vec::new();
        let mut joined_length = 0;

        for mut tagged_text in tagged_texts {
            let text = tagged_text.text();
            joined_text.push_str(&text);
            joined_text.push(delimiter);

            let tags = tagged_text.tags_mut();
            for tag in tags {
                let text_tag = TextTag::new(
                    tag.tag(),
                    tag.start_idx() + joined_length,
                    tag.end_idx() + joined_length,
                );
                joined_tags.push(text_tag);
            }
            joined_length += tagged_text.text().len() + 1;
        }
        TaggedText::new(joined_text, joined_tags)
    }
}
