#[allow(non_snake_case)]
#[allow(dead_code)]
use std::fmt;

#[derive(Clone, Copy)]
pub enum FX {
    Reset = 0,
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Inverse = 7,
    Invisible,
    Strikethrough
}

impl fmt::Display for FX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{}m", *self as u32)
    }
}
#[derive(Default)]
pub struct RGB {
    pub Background: bool,
    pub R: u8,
    pub G: u8,
    pub B: u8
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{}8;2;{};{};{}m", 3 + self.Background as u8, self.R, self.G, self.B)
    }
}
