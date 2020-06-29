pub mod app;
pub mod buffer;
pub mod cursor;
mod nodelist;
pub mod taggedtext;
pub mod texttag;

use taggedtext::TaggedText;
use texttag::TextTag;

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

pub fn apply_syntax_tags(mut tagged_text: TaggedText) -> Vec<TaggedText> {
    if tagged_text.text() == "" {
        return vec![tagged_text];
    }

    let mut to_return = Vec::new();

    let mut text = tagged_text.text_mut().as_str();

    // use this instead of text.lines() to preserve ending empty string
    // to avoid index out of bound error & let cursor to be on new line
    // for e.g. "abc\ndef\n" will be split into ["abc", "def", ""]
    // instead of the default ["abc", "def"] if split by lines()
    let mut lines: Vec<_> = text.split("\n").collect();

    let mut cumulative_lengths = Vec::new();
    cumulative_lengths.push(0);

    for line in lines {
        let string = String::from(line);
        to_return.push(TaggedText::new(string, Vec::new()));

        let last_length = cumulative_lengths.last().unwrap();
        cumulative_lengths.push(last_length + line.len() + 1);
    }

    let mut tags = tagged_text.tags_mut();
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

            let texttag = TextTag::new(
                tag.tag(),
                start - cumulative_lengths[idx],
                end - cumulative_lengths[idx],
            );
            to_return[idx].push_tag(texttag);
        }
    }

    to_return
}

#[cfg(test)]
mod model_tests {
    use super::texttag::*;
    use super::*;

    #[test]
    fn apply_syntax_tags_on_empty_text() {
        let tags = vec![TextTag::new(Tag::Cursor, 0, 1)];
        let mut tagged_text = TaggedText::new(String::new(), tags);

        let result = apply_syntax_tags(tagged_text);

        let expected_tags = vec![TextTag::new(Tag::Cursor, 0, 1)];
        let expected_tagged_text = TaggedText::new(String::new(), expected_tags);

        assert_eq!(result, vec![expected_tagged_text]);
    }

    #[test]
    fn apply_syntax_tags_on_text_across_lines() {
        let tags = vec![TextTag::new(Tag::Cursor, 1, 10)];
        let mut tagged_text = TaggedText::new(String::from("abc\ndef\nghi"), tags);

        let result = apply_syntax_tags(tagged_text);

        let expected_tags_0 = vec![TextTag::new(Tag::Cursor, 1, 3)];
        let expected_tags_1 = vec![TextTag::new(Tag::Cursor, 0, 3)];
        let expected_tags_2 = vec![TextTag::new(Tag::Cursor, 0, 2)];

        let expected_tagged_text_0 = TaggedText::new(String::from("abc"), expected_tags_0);
        assert_eq!(result[0], expected_tagged_text_0);
        let expected_tagged_text_1 = TaggedText::new(String::from("def"), expected_tags_1);
        assert_eq!(result[1], expected_tagged_text_1);
        let expected_tagged_text_2 = TaggedText::new(String::from("ghi"), expected_tags_2);
        assert_eq!(result[2], expected_tagged_text_2);
    }

    #[test]
    fn apply_syntax_tags_on_text_on_same_line() {
        let tags = vec![TextTag::new(Tag::Cursor, 4, 5)];
        let mut tagged_text = TaggedText::new(String::from("abc\ndef\nghi"), tags);

        let result = apply_syntax_tags(tagged_text);

        let expected_tags_0 = Vec::new();
        let expected_tags_1 = vec![TextTag::new(Tag::Cursor, 0, 1)];
        let expected_tags_2 = Vec::new();

        let expected_tagged_text_0 = TaggedText::new(String::from("abc"), expected_tags_0);
        assert_eq!(result[0], expected_tagged_text_0);
        let expected_tagged_text_1 = TaggedText::new(String::from("def"), expected_tags_1);
        assert_eq!(result[1], expected_tagged_text_1);
        let expected_tagged_text_2 = TaggedText::new(String::from("ghi"), expected_tags_2);
        assert_eq!(result[2], expected_tagged_text_2);
    }

    #[test]
    fn apply_syntax_tags_on_text_on_entire_line() {
        let tags = vec![TextTag::new(Tag::Cursor, 4, 7)];
        let mut tagged_text = TaggedText::new(String::from("abc\ndef\nghi"), tags);

        let result = apply_syntax_tags(tagged_text);

        let expected_tags_0 = Vec::new();
        let expected_tags_1 = vec![TextTag::new(Tag::Cursor, 0, 3)];
        let expected_tags_2 = Vec::new();

        let expected_tagged_text_0 = TaggedText::new(String::from("abc"), expected_tags_0);
        assert_eq!(result[0], expected_tagged_text_0);
        let expected_tagged_text_1 = TaggedText::new(String::from("def"), expected_tags_1);
        assert_eq!(result[1], expected_tagged_text_1);
        let expected_tagged_text_2 = TaggedText::new(String::from("ghi"), expected_tags_2);
        assert_eq!(result[2], expected_tagged_text_2);
    }

    #[test]
    fn apply_syntax_tags_on_text_on_entire_text() {
        let tags = vec![TextTag::new(Tag::Cursor, 0, 11)];
        let mut tagged_text = TaggedText::new(String::from("abc\ndef\nghi"), tags);

        let result = apply_syntax_tags(tagged_text);

        let expected_tags_0 = vec![TextTag::new(Tag::Cursor, 0, 3)];
        let expected_tags_1 = vec![TextTag::new(Tag::Cursor, 0, 3)];
        let expected_tags_2 = vec![TextTag::new(Tag::Cursor, 0, 3)];

        let expected_tagged_text_0 = TaggedText::new(String::from("abc"), expected_tags_0);
        assert_eq!(result[0], expected_tagged_text_0);
        let expected_tagged_text_1 = TaggedText::new(String::from("def"), expected_tags_1);
        assert_eq!(result[1], expected_tagged_text_1);
        let expected_tagged_text_2 = TaggedText::new(String::from("ghi"), expected_tags_2);
        assert_eq!(result[2], expected_tagged_text_2);
    }
}
