use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::event::Key;

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
    		Ok(QuitOption::NotQuitting) => {},
    		Err(x) => panic!("{:?}", x),
    	};
    }

    Ok(())
}

fn handle_event(event: Event, app: &mut App) -> Result<QuitOption, ()> {
	match event {
		Event::Tick{..} => Ok(QuitOption::NotQuitting),
		Event::Input{key: Key::Char('q'), ..} => Ok(QuitOption::Quitting),
		Event::Input{key: Key::Char(c), ..} => {
			app.add_char(c);
			Ok(QuitOption::NotQuitting)
		},
		Event::Input{key: Key::Backspace, ..} => {
			app.remove_char();
			Ok(QuitOption::NotQuitting)
		},
		_ => Ok(QuitOption::NotQuitting),
	}

}







