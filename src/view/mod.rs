use std::io;

use crate::model::app::{App, AppMode, CommandMode};
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

    pub fn update_display(&mut self, app: &App) -> Result<(), io::Error> {
        let text = app.get_text_as_iter(); // Get a copy of the text to be rendered
                                           // For now let's not do anything fancy formatting
        let text: Vec<_> = text.iter().map(|x| Text::raw(x)).collect();

        let title = match app.app_mode() {
            AppMode::Command(CommandMode::Write) => "Command mode: Saving into file",
            AppMode::Command(CommandMode::Read) => "Command mode: Opening a file",
            AppMode::Edit => "Edit mode",
        };

        self.terminal.draw(|mut f| {
            let size = f.size();
            let block = Paragraph::new(text.iter())
                .block(Block::default().title(title).borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left)
                .wrap(true);
            f.render_widget(block, size);
        })?;
        Ok(())
    }
}
