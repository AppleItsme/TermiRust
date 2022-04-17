#![allow(dead_code)]
#![allow(non_snake_case)]
mod lib;
use lib::draw;

fn main() {
    let mut cursor = match draw::Cursor::new([10, 20]) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    cursor.draw_square([[1,1],[20,20]], Some(draw::colours::RGB{g:255, background: true, ..Default::default()}), None, String::from(" "));
    cursor.draw_square([[5, 5],[20, 20]], Some(draw::colours::RGB{r:255, background:true, ..Default::default()}), Some(draw::colours::FX::Blink), String::from("y"));
}
