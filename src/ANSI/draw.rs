use std::fmt;

pub struct GoTo(pub u16, pub u16);
pub struct Show;
pub struct Hide;
pub struct Position {
    pub x: i32,
    pub y: i32
}


macro_rules! CursorMovement {
    ($name:ident, $codeLetter:expr) => {
        struct $name(u16);
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "\x1B[{}{}", self.0, $codeLetter)
            }
        }
    };
}

CursorMovement!(CursorLeft, "D");
CursorMovement!(CursorRight, "C");
CursorMovement!(CursorUp, "B");
CursorMovement!(CursorDown, "A");

impl Position {
    pub fn MoveCursor(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
        if x < 0 {
            print!("{}", CursorLeft((x as i16 * -1) as u16));
        } else if x > 0 {
            print!("{}", CursorRight(x as u16));
        }

        if y > 0 {
            print!("{}", CursorUp(y as u16));
        } else if y < 0 {
            print!("{}", CursorDown((y as i16 * -1) as u16));
        }
    }
}

impl fmt::Display for Show {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1B[?25h")
    }
}


impl fmt::Display for Hide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[?25l")
    }
}
