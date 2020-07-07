use std::collections::HashMap;
use std::io;

use crate::model::app::{App, AppMode, CommandMode};
use crate::model::taggedtext::TaggedText;
use crate::model::texttag::*;

#[allow(unused_imports)]
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, Paragraph, Text},
    Terminal,
};

#[derive(Debug)]
pub struct StyleFunc {
    pub f: fn(Style) -> Style,
}

pub struct View<B: Backend> {
    terminal: Terminal<B>,
    // Probably want to include layout details
    //  here eventually.
    tag_to_func: HashMap<Tag, StyleFunc>,
    y_offset: u16,
    x_offset: u16,
}

impl<B: Backend> View<B> {
    pub fn new(terminal: Terminal<B>, tag_to_func: HashMap<Tag, StyleFunc>) -> View<B> {
        View {
            terminal: terminal,
            tag_to_func: tag_to_func,
            y_offset: 0,
            x_offset: 0,
        }
    }

    pub fn update_offset(&mut self, y_offset: u16, x_offset: u16) {
        self.y_offset = y_offset;
        self.x_offset = x_offset;
    }

    pub fn update_display(&mut self, app: &App) -> Result<(), io::Error> {
        let (tagged_texts, title) = match app.mode() {
            AppMode::Command(CommandMode::Read) => (
                app.get_command_buffer_as_tagged_text_vector(),
                "Command mode: Opening a file",
            ),
            AppMode::Command(CommandMode::Write) => (
                app.get_command_buffer_as_tagged_text_vector(),
                "Command mode: Saving into file",
            ),
            AppMode::Edit => (app.get_buffer_as_tagged_text_vector(), "Edit mode"),
        };
        let size = self.terminal.get_frame().size();
        let (mut y_offset, mut x_offset) = (self.y_offset, self.x_offset);

        // TODO: allow user input configuration for wrapping
        // Currently, default is no wrap
        let scrolled_text = scroller::render(&mut y_offset, &mut x_offset, tagged_texts, size, false);
        let text: Vec<_> = highlighter::highlight_tagged_text(&scrolled_text, &self.tag_to_func);
     
        self.terminal.draw(|mut f| {
            let block = Paragraph::new(text.iter())
                .block(Block::default().title(title).borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left)
                .wrap(false);
            f.render_widget(block, size);
        })?;
        self.update_offset(y_offset, x_offset);
        self.terminal.hide_cursor()?;

        Ok(())
    }
}

mod scroller {
    use tui::layout::Rect;

    use unicode_width::UnicodeWidthStr;
    use unicode_width::UnicodeWidthChar;

    use std::convert::TryInto;

    use crate::model::taggedtext::TaggedText;
    use crate::model::texttag::{TextTag, Tag};

    /* Supported directions of scrolling */
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ScrollDirection {
        Up,
        Down,
        Left,
        Right,
    }

    pub fn render(
        y_offset: &mut u16, x_offset: &mut u16, 
        tagged_texts: Vec<TaggedText>, 
        window: Rect, wrap: bool
    ) -> TaggedText {

        let mut split_text: Vec<Vec<TaggedText>> = Vec::new();
        for line in tagged_texts {
            split_text.push(line.split_whitespace());
        }

        let mut selected_texts = Vec::new();
        if wrap {
            selected_texts = select_with_wrap(y_offset, x_offset, split_text, window);
        } else {
            selected_texts = select_without_wrap(y_offset, x_offset, split_text, window);
        }

        TaggedText::join(selected_texts, '\n')                            
    }

    /**
     * Scroll text in the specified direction by the given amount.
     * 
     * @param direction
     * @param scroll_amount
     */
    fn scroll(y_offset: &mut u16, x_offset: &mut u16, direction: ScrollDirection, scroll_amount: u16) {
        match direction {
            ScrollDirection::Up => {
                *y_offset += scroll_amount;
            }

            ScrollDirection::Down => {
                *y_offset = match (*y_offset).checked_sub(scroll_amount) {
                    Some(v) => v,
                    None => 0,
                }
            }

            ScrollDirection::Left => {
                *x_offset = match (*x_offset).checked_add(scroll_amount) {
                    Some(v) => v,
                    None => 0,
                }
            }

            ScrollDirection::Right => {
                *x_offset = match (*x_offset).checked_sub(scroll_amount) {
                    Some(v) => v,
                    None => 0,
                }
            }

            _ => {}
        }
    }

    /**
     * Select a portion of text to fit the window area (with wrapping)
     * 
     * @param: tagged_texts
     * 
     * @return: selected_texts
     */
    fn select_with_wrap(y_offset: &mut u16, x_offset: &mut u16, tagged_texts: Vec<Vec<TaggedText>>, window: Rect) -> Vec<TaggedText> {
        // TODO: fix bug when moving up/down wrapped text
        // need to split into multiple lines
        
        let mut x: u16 = 0; // current line width
        let mut y: u16 = 0; // current displayed line
        let mut selected_texts = Vec::new();
        let max_height = window.height - 3; // compensate for terminal border
        let max_width = window.width - 1; // compensate for terminal border
        
        for line in tagged_texts {
            let mut selected_line = Vec::new();

            for word in line {

                let add_width = (word.text().width() + " ".width()) as u16;
                if x + add_width > max_width {
                    selected_texts.push(selected_line);
                    selected_line = Vec::new();
                    selected_line.push(word);
                    x = add_width;
                    y += 1;
                    continue;
                }

                let cursors: Vec<TextTag> = word.tags().iter()
                                                        .map(|x| *x)
                                                        .filter(|x| x.tag() == Tag::Cursor)
                                                        .collect();

                if cursors.len() > 0 {
                    if y < *y_offset {
                        scroll(y_offset, x_offset, ScrollDirection::Down, *y_offset - y);
                    }

                    if y > *y_offset + max_height {
                        scroll(y_offset, x_offset, ScrollDirection::Up, y - *y_offset - max_height);
                    }
                }
                
                x += add_width;
                selected_line.push(word);

            }

            selected_texts.push(selected_line);
            selected_line = Vec::new();
            x = 0;
            y += 1;
        }

        for i in 0..*y_offset {
            selected_texts.remove(0);
        }

        selected_texts.into_iter().map(|x| TaggedText::join(x, ' ')).collect()
    }

    /**
     * Select a portion of text to fit the window area (without wrapping)
     * 
     * @param: tagged_texts
     * 
     * @return: to_return
     */
    fn select_without_wrap(y_offset: &mut u16, x_offset: &mut u16, tagged_texts: Vec<Vec<TaggedText>>, window: Rect) -> Vec<TaggedText> {
        let mut x: u16 = 0; // current line width
        let mut y: u16 = 0; // current displayed line
        let mut selected_texts = Vec::new();
        let max_height = window.height - 3; // compensate for terminal border
        let max_width = window.width - 1; // compensate for terminal border

        for line in tagged_texts {
            let mut selected_line = Vec::new();

            for word in line {
                let add_width = (word.text().width() + " ".width()) as u16;
                x += add_width;

                let cursors: Vec<TextTag> = word.tags().iter()
                                                        .map(|x| *x)
                                                        .filter(|x| x.tag() == Tag::Cursor)
                                                        .collect();
                if cursors.len() > 0 {

                    let cursor = cursors.get(0).unwrap();
                    let cursor_start_idx = x - add_width + cursor.start_idx() as u16;
                    let cursor_end_idx = x - add_width + cursor.end_idx() as u16;

                    if cursor_start_idx < *x_offset {
                        scroll(y_offset, x_offset, ScrollDirection::Right, *x_offset - cursor_start_idx);
                    }

                    if cursor_end_idx >= *x_offset + max_width {
                        scroll(y_offset, x_offset, ScrollDirection::Left, cursor_end_idx - *x_offset - max_width + 1);
                    }

                    if y < *y_offset {
                        scroll(y_offset, x_offset, ScrollDirection::Down, *y_offset - y);
                    }

                    if y > *y_offset + max_height {
                        scroll(y_offset, x_offset, ScrollDirection::Up, y - *y_offset - max_height);
                    }
                }

                selected_line.push(word);
            }

            selected_texts.push(selected_line);
            selected_line = Vec::new();
            x = 0;
            y += 1;
        }

        for i in 0..*y_offset{
            selected_texts.remove(0);
        }

        let mut to_return: Vec<TaggedText> = selected_texts.iter().map(|x| TaggedText::join(x.to_vec(), ' ')).collect();

        if *x_offset > 0 {
            let skip: usize = (*x_offset).try_into().unwrap();
            for mut line in &mut to_return {
                let text_mut = line.text_mut();
                *text_mut = text_mut.split_off(skip);

                let tags_mut = line.tags_mut();
                for tag in tags_mut {
                    *tag = TextTag::new(tag.tag(), tag.start_idx() - skip, tag.end_idx() - skip);
                }
            }
        }

        to_return
    }
}

mod highlighter {
    use std::collections::HashMap;
    use std::collections::HashSet;

    use std::iter::IntoIterator;

    use super::StyleFunc;
    use crate::model::taggedtext::TaggedText;
    use crate::model::texttag::*;

    use tui::style::{Color, Style};
    use tui::widgets::Text;

    pub fn highlight_tagged_text<'a>(
        text: &'a TaggedText,
        tag_to_func: &HashMap<Tag, StyleFunc>,
    ) -> Vec<Text<'a>> {
        let mut highlighted_text: Vec<Text> = Vec::new();

        // break the boundaries of the Tags into two queues
        let tags = text.tags();
        let tagged_str = text.as_str();
        let mut start_indices = Vec::new();
        let mut end_indices = Vec::new();
        for tag in tags {
            let start_idx = tag.start_idx();
            let end_idx = tag.end_idx();

            start_indices.push((start_idx, tag.tag()));
            end_indices.push((end_idx, tag.tag()));
        }

        let mut active_tags = HashSet::new();
        let mut last_idx = 0;
        let mut left = 0;
        let mut right = 0;
        while left < start_indices.len() || right < end_indices.len() || last_idx < tagged_str.len()
        {
            let mut chunk_str = "";

            if left == start_indices.len() && right == end_indices.len() {
                chunk_str = &tagged_str[last_idx..tagged_str.len()];
                let styled_chunk =
                    Text::styled(chunk_str, style_from_tags(&active_tags, tag_to_func));
                highlighted_text.push(styled_chunk);
                break;
            }

            if last_idx == tagged_str.len() {
                // Cursor should be the only tag at the end of the string
                active_tags.clear();
                active_tags.insert(Tag::Cursor);
                let styled_chunk =
                    Text::styled("█", Style::default().bg(Color::Black).fg(Color::White));
                highlighted_text.push(styled_chunk);
                break;
            }

            if left == start_indices.len() || end_indices[right].0 < start_indices[left].0 {
                let next_pair = end_indices[right];
                let next_idx = next_pair.0;
                let next_tag = next_pair.1;

                chunk_str = &tagged_str[last_idx..next_idx];
                if next_idx > last_idx {
                    if chunk_str.eq_ignore_ascii_case("\n") && next_tag == Tag::Cursor {
                        let styled_chunk =
                            Text::styled("█", Style::default().bg(Color::Black).fg(Color::White));
                        highlighted_text.push(styled_chunk);
                    }
                    let styled_chunk =
                        Text::styled(chunk_str, style_from_tags(&active_tags, tag_to_func));
                    highlighted_text.push(styled_chunk);
                }
                active_tags.remove(&next_tag);
                last_idx = next_idx;
                right += 1;
            } else {
                let next_pair = start_indices[left];
                let next_idx = next_pair.0;
                let next_tag = next_pair.1;

                chunk_str = &tagged_str[last_idx..next_idx];
                if next_idx > last_idx {
                    if chunk_str.eq_ignore_ascii_case("\n") && next_tag == Tag::Cursor {
                        let styled_chunk =
                            Text::styled("█", Style::default().bg(Color::Black).fg(Color::White));
                        highlighted_text.push(styled_chunk);
                    }
                    let styled_chunk =
                        Text::styled(chunk_str, style_from_tags(&active_tags, tag_to_func));
                    highlighted_text.push(styled_chunk);
                }
                active_tags.insert(next_tag);
                last_idx = next_idx;
                left += 1;
            }
        }
        highlighted_text
    }

    fn style_from_tags(active_tags: &HashSet<Tag>, tag_to_func: &HashMap<Tag, StyleFunc>) -> Style {
        let initial_style = if active_tags.contains(&Tag::Cursor) {
            Style::default().fg(Color::Black).bg(Color::White)
        } else {
            Style::default()
        };
        active_tags
            .iter()
            .fold(initial_style, |acc, tag| match tag_to_func.get(&tag) {
                Some(func_struct) => (func_struct.f)(acc),
                None => acc,
            })
    }

    #[cfg(test)]
    mod highlighted_tests {
        use super::*;

        #[test]
        fn highlight_empty_text() {
            let mut tag_to_func = HashMap::new();
            tag_to_func.insert(Tag::Cursor, StyleFunc { f: |arg_1| arg_1 });
            let text = String::new();
            let tags = vec![TextTag::new(Tag::Cursor, 0, 1)];
            let tagged_text = TaggedText::new(text, tags);

            let text = highlight_tagged_text(&tagged_text, &tag_to_func);
            assert_eq!(
                text,
                [Text::styled(
                    "█",
                    Style::default().fg(Color::White).bg(Color::Black)
                )]
            );
        }

        #[test]
        fn highlight_single_char() {
            let mut tag_to_func = HashMap::new();
            tag_to_func.insert(Tag::Cursor, StyleFunc { f: |arg_1| arg_1 });
            let text = String::from("a");
            let tag_0 = TextTag::new(Tag::Cursor, 1, 2);
            let tags = vec![tag_0];
            let tagged_text = TaggedText::new(text, tags);

            let text = highlight_tagged_text(&tagged_text, &tag_to_func);
            let expected_text = [
                Text::styled("a", Style::default()),
                Text::styled("█", Style::default().fg(Color::White).bg(Color::Black)),
            ];
            assert_eq!(text, expected_text);
        }

        #[test]
        fn highlight_some_text() {
            let mut tag_to_func = HashMap::new();
            tag_to_func.insert(
                Tag::Cursor,
                StyleFunc {
                    f: |arg_1| Style::bg(arg_1, Color::White).fg(Color::Black),
                },
            );
            let text = String::from("\nbcde\nghi");
            let tag_0 = TextTag::new(Tag::Cursor, 4, 5);
            let tags = vec![tag_0];
            let tagged_text = TaggedText::new(text, tags);

            let text = highlight_tagged_text(&tagged_text, &tag_to_func);
            let expected_text = [
                Text::styled("\nbcd", Style::default()),
                Text::styled("e", Style::default().fg(Color::Black).bg(Color::White)),
                Text::styled("\nghi", Style::default()),
            ];
            assert_eq!(text, expected_text);
        }
    }
}
