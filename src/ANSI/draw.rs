pub mod colours;

use std::io::{self, Write, Error};
pub struct Cursor {
    resolution: [u16;2]
}
pub enum Direction {
    Up = 0,
    Down,
    Right,
    Left
}

impl Cursor {
    pub fn new(boundary: [u16;2]) -> Result<Cursor, Error> {
        let mut handle = io::stdout();
        match handle.write("\x1b[H".as_bytes()) {
            Ok(_) => {},
            Err(e) => return Err(e),
        }
        return match handle.flush() {
            Ok(_) => Ok(Cursor{resolution: boundary}),
            Err(e) => Err(e),
        }
    }
    pub fn goto(&mut self, x: u16, y: u16){
        self.print(format!("\x1b[{};{}H", y, x));
    }
    pub fn print<T: Into<String>>(&mut self, txt: T) {
        let mut handle = io::stdout();
        let text = txt.into();
        handle.write(text.as_bytes()).unwrap();
        handle.flush().unwrap();
    }
    pub fn cursor_move(&mut self, n: u16, direction: Direction) {
        let dir_arr = ["A", "B", "C", "D"];
        let dir_to_num = direction as usize;
        self.print(format!("\x1b[{}{}", n, dir_arr[dir_to_num]));
    }
    pub fn draw_square(&mut self, coords: [[u16;2];2], colour: Option<colours::RGB>, fx: Option<colours::FX>, text: String) {
        if colour.is_some() {
             self.print(format!("{}", colour.unwrap()));
        }
        if fx.is_some() {
            self.print(format!("{}", fx.unwrap()));
        }
        self.goto(coords[0][0], coords[0][1]);
        for y in coords[0][1]..coords[1][1]+2 {
            for x in coords[0][0]..coords[1][0]+1 {
                if x > self.resolution[0] { break; }
                self.print((&text).to_owned()); 
            }
            self.print("\n".to_string());
            if y > self.resolution[1]+1 { 
                break; 
            }
            self.goto(coords[0][0], y); 
        }
        self.print("\n".to_string());
    }
}
