#![allow(dead_code)]
#![allow(non_snake_case)]

#[path="./ANSI/draw.rs"]
mod draw;
#[path="./ANSI/colours.rs"]
mod colours;

use colours::RGB;

fn main() {
    let mut cursor = draw::Cursor::new();
    cursor.goto(1, 1);
    print!("hallo");
    loop {};
}
