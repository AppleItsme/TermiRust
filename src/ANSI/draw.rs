use std::io::{self, Write, Error};
pub struct Cursor;

pub enum Direction {
    Up = 0,
    Down,
    Right,
    Left
}

impl Cursor {
    pub fn new() -> Result<Cursor, Error> {
        let mut handle = io::stdout();
        match handle.write("\x1b[H".as_bytes()) {
            Ok(_) => {},
            Err(e) => return Err(e),
        }
        match handle.flush() {
            Ok(_) => {},
            Err(e) => return Err(e),
        }
        let mut cursor = Cursor;
        return match cursor.goto(1, 1) {
           Ok(_) => Ok(cursor),
           Err(e) => Err(e),
        }
    }
    pub fn goto(&mut self, x: u16, y: u16) -> Result<(), Error> {
        let result = self.print(format!("\x1b[{};{}H", y, x));
        result
    }
    pub fn print(&mut self, txt: String) -> Result<(), Error> {
        let mut handle = io::stdout();
        let result = handle.write(txt.as_bytes());
        match result {
            Err(e) => return Err(e),
            Ok(_) => {},
        }
        handle.write("\0".as_bytes())?;
        handle.flush()
    }
    pub fn cursor_move(&mut self, n: u16, direction: Direction) -> Result<(), Error>{
        let dir_arr = ["A", "B", "C", "D"];
        let dir_to_num = direction as usize;
        self.print(format!("\x1b[{}{}", n, dir_arr[dir_to_num]))
    }
}
