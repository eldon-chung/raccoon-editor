use std::io;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::Terminal;

mod utils;
use crate::utils::events::{Event, Events};
use crate::utils::QuitOption;

mod model;
use crate::model::app::App;

mod view;
use crate::view::View;

fn main() -> Result<(), io::Error> {
    println!("Hello, world!");
    // Setup buffers, load configs
    // Construct program state
    let mut app: App = App::new();

    // Construct the event queue
    let events = Events::new();

    // Enter raw mode
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut view = View::new(terminal);

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
        },
        Event::Input{key: Key::Ctrl('s'), ..} => {
            app.save_file();
            Ok(QuitOption::Quitting)
        },
        Event::Input{key: Key::Ctrl('o'), ..} => {
            // Currently, it opens a foo.txt
            // However, should change this to upon pressing,
            // user gets a prompt that will ask them to enter filename
            app.open_file();
            Ok(QuitOption::NotQuitting)
        },
        _ => Ok(QuitOption::NotQuitting),
    }
}
