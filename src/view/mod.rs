use std::io;

use crate::model::app::App;

#[allow(unused_imports)]
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, Paragraph, Text},
    Terminal,
};

pub struct View<B: Backend> {
    terminal: Terminal<B>,
    // Probably want to include layout details
    //  here eventually.
}

impl<B: Backend> View<B> {
    pub fn new(terminal: Terminal<B>) -> View<B> {
        View { terminal: terminal }
    }

    /*
    pub fn update_display(&mut self, app: &App) -> Result<(), io::Error> {
        let text = app.get_text_as_iter(); // Get a copy of the text to be rendered
                                           // For now let's not do anything fancy formatting
        let text: Vec<_> = text.iter().map(|x| Text::raw(x)).collect();
        self.terminal.draw(|mut f| {
            let size = f.size();
            let block = Paragraph::new(text.iter())
                .block(Block::default().title("Paragraph").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left)
                .wrap(true);
            f.render_widget(block, size);
        })?;

        Ok(())
    }*/

    pub fn update_display(&mut self, app: &App) -> Result<(), io::Error> {
        let tagged_text = app.get_tagged_text(); // Get a copy of the text to be rendered
                                                 // For now let's not do anything fancy formatting
        let text: Vec<_> = Highlighter::highlight_tagged_text(&tagged_text);
        eprintln!("highlighted_text: {:?}", text);
        self.terminal.draw(|mut f| {
            let size = f.size();
            let block = Paragraph::new(text.iter())
                .block(Block::default().title("Paragraph").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left)
                .wrap(true);
            f.render_widget(block, size);
        })?;
        self.terminal.hide_cursor();

        Ok(())
    }
}

mod Highlighter {
    use std::collections::HashMap;
    use std::collections::HashSet;

    use std::iter::IntoIterator;

    use crate::model::taggedtext::TaggedText;
    use crate::model::texttag::*;

    use tui::style::{Color, Style};
    use tui::widgets::Text;

    pub fn highlight_tagged_text(text: &TaggedText) -> Vec<Text> {
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
                let styled_chunk = Text::styled(chunk_str, style_from_tags(&active_tags));
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
                    let styled_chunk = Text::styled(chunk_str, style_from_tags(&active_tags));
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
                    let styled_chunk = Text::styled(chunk_str, style_from_tags(&active_tags));
                    highlighted_text.push(styled_chunk);
                }
                active_tags.insert(next_tag);
                last_idx = next_idx;
                left += 1;
            }
        }
        highlighted_text
    }

    #[derive(Debug)]
    struct StyleFunc {
        f: fn(Style) -> Style,
    }

    fn style_from_tags(list: &HashSet<Tag>) -> Style {
        // for now just make the mapping per call
        let mut tag_to_func = HashMap::new();
        tag_to_func.insert(
            Tag::Cursor,
            StyleFunc {
                f: |arg_1| Style::bg(arg_1, Color::White).fg(Color::Black),
            },
        );

        let initial_style = Style::default();
        let initial_func: Box<dyn Fn(Style) -> Style> = Box::new(|x| x);
        let final_func = list.iter().fold(initial_func, |acc, tag| {
            compose(acc, tag_to_func.get(&tag).unwrap().f)
        });
        final_func(initial_style)
    }

    fn compose<'f, F1, F2>(f1: F1, f2: F2) -> Box<dyn Fn(Style) -> Style + 'f>
    where
        F1: Fn(Style) -> Style + 'f,
        F2: Fn(Style) -> Style + 'f,
    {
        Box::new(move |input| f2(f1(input)))
    }

    #[cfg(test)]
    mod highlighted_tests {
        use super::*;

        #[test]
        fn highlight_empty_text() {
            let text = String::new();
            let tags = vec![TextTag::new(Tag::Cursor, 0, 1)];
            let tagged_text = TaggedText::new(text, tags);

            let text = highlight_tagged_text(&tagged_text);
            assert_eq!(
                text,
                [Text::styled(
                    " ",
                    Style::default().fg(Color::Black).bg(Color::White)
                )]
            );
        }

        #[test]
        fn highlight_single_char() {
            let text = String::from("a");
            let tag_0 = TextTag::new(Tag::Cursor, 1, 2);
            let tags = vec![tag_0];
            let tagged_text = TaggedText::new(text, tags);

            let text = highlight_tagged_text(&tagged_text);
            let expected_text = [
                Text::styled("a", Style::default()),
                Text::styled(" ", Style::default().fg(Color::Black).bg(Color::White)),
            ];
            assert_eq!(text, expected_text);
        }

        #[test]
        fn highlight_some_text() {
            let text = String::from("\nbcde\nghi");
            let tag_0 = TextTag::new(Tag::Cursor, 4, 5);
            let tags = vec![tag_0];
            let tagged_text = TaggedText::new(text, tags);

            let text = highlight_tagged_text(&tagged_text);
            let expected_text = [
                Text::styled("\nbcd", Style::default()),
                Text::styled("e", Style::default().fg(Color::Black).bg(Color::White)),
                Text::styled("\nghi", Style::default()),
            ];
            assert_eq!(text, expected_text);
        }
    }
}
