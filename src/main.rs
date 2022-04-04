#![allow(dead_code)]
#![allow(non_snake_case)]

#[path="./ANSI/draw.rs"]
mod draw;
#[path="./ANSI/colours.rs"]
mod colours;

use colours::RGB;
use draw::Position;

fn main() {
    let mut pos = Position { x: 1, y: 1 };
    println!("{}haaalloooooo{}", colours::FX::Blink, colours::FX::Reset);
    pos.MoveCursor(10, 10);
    println!("{}h{}e{}r{}e{} :D", 
        RGB { R: 255, ..Default::default() },
        colours::FX::Inverse,
        RGB { G: 255, ..Default::default() },
        colours::FX::Underline,
        RGB { R: 255, G: 255, B: 255, ..Default::default() }
    );
    println!("x: {} y: {}", pos.x, pos.y);
}
