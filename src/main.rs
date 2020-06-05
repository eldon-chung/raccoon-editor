use std::collections::HashMap;
use std::env;
use std::io;

use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::style::{Color, Style};
use tui::Terminal;

mod utils;
use crate::utils::events::{Event, Events};
use crate::utils::QuitOption;

mod model;
use crate::model::app::{App, AppMode, CommandMode};
use crate::model::taggedtext::TaggedText;
use crate::model::texttag::*;

mod view;
use crate::view::*;

fn main() -> Result<(), io::Error> {
    // Setup buffers, load configs
    // Construct program state
    let args: Vec<String> = env::args().collect();
    let mut app: App = App::new(&args);

    // Load the view configs
    let mut tag_to_func = HashMap::new();
    // TODO: eventually this should be loading such configurations from a file
    tag_to_func.insert(Tag::Cursor, view::StyleFunc { f: |arg_1| arg_1 });

    // Construct the event queue
    let events = Events::new();

    // Enter raw mode
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut view = View::new(terminal, tag_to_func);

    // Loop:
    // get next event from event queue
    // handle next event, update program state
    loop {
        view.update_display(&app)?;

        let event = match events.next() {
            Ok(event) => event,
            Err(e) => panic!("{:?}", e),
        };

        match handle_event(event, &mut app) {
            Ok(QuitOption::Quitting) => break,
            Ok(QuitOption::NotQuitting) => {}
            Err(x) => panic!("{:?}", x),
        };
    }

    Ok(())
}

fn handle_event(event: Event, app: &mut App) -> Result<QuitOption, ()> {
    match event {
        // Full list of keys can be found at
        // https://docs.rs/termion/1.1.1/termion/event/enum.Key.html
        Event::Tick { .. } => Ok(QuitOption::NotQuitting),
        Event::Input {
            key: Key::Char('q'),
            ..
        } => Ok(QuitOption::Quitting),
        Event::Input {
            key: Key::Char('\n'),
            ..
        } => match app.app_mode() {
            // If in command mode, pressing enter means you want
            // to enter the command
            AppMode::Command(CommandMode::Write) => {
                app.save_file();
                Ok(QuitOption::NotQuitting)
            }
            AppMode::Command(CommandMode::Read) => {
                app.open_file();
                Ok(QuitOption::NotQuitting)
            }
            _ => {
                // in Edit mode, it means you want to insert the newline character
                app.add_char('\n');
                Ok(QuitOption::NotQuitting)
            }
        },
        Event::Input {
            key: Key::Char(c), ..
        } => {
            app.add_char(c);
            Ok(QuitOption::NotQuitting)
        }
        Event::Input {
            key: Key::Backspace,
            ..
        } => {
            app.remove_char();
            Ok(QuitOption::NotQuitting)
        }
        Event::Input {
            key: Key::Right, ..
        } => {
            app.move_cursor_right();
            Ok(QuitOption::NotQuitting)
        }
        Event::Input { key: Key::Left, .. } => {
            app.move_cursor_left();
            Ok(QuitOption::NotQuitting)
        }
        Event::Input { key: Key::Up, .. } => {
            app.move_cursor_up();
            Ok(QuitOption::NotQuitting)
        }
        Event::Input { key: Key::Down, .. } => {
            app.move_cursor_down();
            Ok(QuitOption::NotQuitting)
        }
        Event::Input {
            key: Key::Ctrl('s'),
            ..
        } => {
            app.handle_regular_save();
            Ok(QuitOption::NotQuitting)
        }
        Event::Input {
            // This should be replaced with Ctrl+Shift+S. Ctrl + a is temporary workaround
            key: Key::Ctrl('a'),
            ..
        } => {
            app.handle_save_as_new_file();
            Ok(QuitOption::NotQuitting)
        }
        Event::Input {
            key: Key::Ctrl('o'),
            ..
        } => {
            app.set_app_mode(AppMode::Command(CommandMode::Read));
            Ok(QuitOption::NotQuitting)
        }
        _ => Ok(QuitOption::NotQuitting),
    }
}
