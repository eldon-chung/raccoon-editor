use std::io;
use std::sync::mpsc::{
    self,
    Sender
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::{Duration, SystemTime};

use termion::event::Key;
use termion::input::TermRead;

pub enum Event {
    Tick {time: SystemTime},
    Input {key: Key, time: SystemTime}
}

pub struct Events {
    rx: mpsc::Receiver<Event>,
    input_handle: thread::JoinHandle<()>,
    ignore_exit_key: Arc<AtomicBool>,
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

#[allow(dead_code)]
impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));

        let input_handle = {
            let tx = tx.clone();
            let ignore_exit_key = ignore_exit_key.clone();
            thread::spawn(move || Events::input_thread(ignore_exit_key, tx, config.exit_key))
        };

        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || Events::tick_thread(tx, config.tick_rate))
        };
        Events {
            rx,
            ignore_exit_key,
            input_handle,
            tick_handle,
        }
    }

    fn input_thread(ignore_exit_key: Arc<AtomicBool>, tx: Sender<Event>, exit_key: Key) {
        let stdin = io::stdin();
        for evt in stdin.keys() {
            match evt {
                Ok(key) => {
                    let to_send = Event::Input {
                        key: key,
                        time: SystemTime::now(),
                    };

                    if let Err(_) = tx.send(to_send) {
                        return;
                    }
                    if !ignore_exit_key.load(Ordering::Relaxed) && key == exit_key {
                        return;
                    }
                }
                Err(_) => {}
            }
        }
    }

    fn tick_thread(tx: Sender<Event>, tick_rate: Duration) {
        let tx = tx.clone();
        loop {
            let to_send = Event::Tick {
                time: SystemTime::now(),
            };
            tx.send(to_send).unwrap();
            thread::sleep(tick_rate);
        }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }

    pub fn disable_exit_key(&mut self) {
        self.ignore_exit_key.store(true, Ordering::Relaxed);
    }

    pub fn enable_exit_key(&mut self) {
        self.ignore_exit_key.store(false, Ordering::Relaxed);
    }
}