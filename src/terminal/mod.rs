pub mod key;

use crossterm::terminal;
use key::Key;
use std::{
    io::{self, Stdin},
    sync::mpsc::{self, Receiver},
    thread,
};

pub struct RawTerminal {
    reader: Option<Receiver<Key>>,
}

impl RawTerminal {
    pub fn new() -> Self {
        terminal::enable_raw_mode().unwrap();

        let mut zelf = Self { reader: None };
        zelf.capture_stdin();

        zelf
    }

    pub fn println(&self, msg: String) {
        println!("{}\r", msg);
    }

    pub fn key(&self) -> Option<Key> {
        if let Some(reader) = &self.reader {
            let key = reader.try_recv().unwrap_or_default();
            if key == Key::Exit {
                None
            } else {
                Some(key)
            }
        } else {
            Some(Key::default())
        }
    }

    pub fn capture_stdin(&mut self) {
        let (tx, rx) = mpsc::channel();
        let mut stdin = io::stdin();

        thread::spawn(move || loop {
            tx.send(stdin.read()).unwrap();
        });

        self.reader = Some(rx);
    }

    pub fn clear(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}

pub trait ReadKey {
    fn read(&mut self) -> Key;
}

impl ReadKey for Stdin {
    fn read(&mut self) -> Key {
        use io::Read;

        let mut buf = [0; 1];
        Read::read(self, &mut buf).expect("Failed to read line");

        if buf != [27] {
            buf[0].into()
        } else {
            let mut buf2 = [0; 1];
            Read::read(self, &mut buf2).expect("Failed to read line");
            let mut buf3 = [0; 1];
            Read::read(self, &mut buf3).expect("Failed to read line");

            [buf[0], buf2[0], buf3[0]].into()
        }
    }
}
