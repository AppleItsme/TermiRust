#![allow(dead_code)]
#![allow(non_snake_case)]
mod lib;
use lib::draw;
use lib::colours;

fn main() {
    let mut cursor = match draw::Cursor::new() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    cursor.print("\x1b[=18h".to_string()).unwrap();
    cursor.draw_square([[1,1], [10, 12]], Some(colours::RGB{r:255, g:255, b:255, background:true}), None, " ".to_string());
    cursor.draw_square([[3, 3], [5, 10]], Some(colours::RGB{r: 255, background: true, ..Default::default()}), None, " ".to_string());
}
