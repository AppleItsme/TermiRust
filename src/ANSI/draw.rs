use std::{io::{self, Write}, error::Error};

pub struct Cursor {
    pub x: u16,
    pub y: u16
}

impl Cursor {
    pub fn new() -> Cursor {
        let mut handle = io::stdout();
        let result = handle.write("\x1b[H".as_bytes());
        let mut err = result.err();
        if err.is_some() { eprint!("{:?}",err); }
        err = handle.flush().err();
        if err.is_some() { eprint!("{:?}", err); }
        Cursor{ x: 1, y: 1 }
    }

    pub fn goto(&mut self, X: u16, Y: u16) -> Option<io::Error> {
        self.x = X;
        self.y = Y;
        let mut handle = io::stdout();
        let result = handle.write(format!("\x1B[{};{}H", X, Y).as_bytes());
        match &result {
            Ok(_) => {},
            Err(_) => return result.err(),
        }
        handle.flush().err()
    }
    pub fn print(&mut self, txt: &str) {

    }
}
