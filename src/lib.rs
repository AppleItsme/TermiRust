#[path="./ANSI/colours.rs"]
pub mod colours;
#[path="./ANSI/draw.rs"]
pub mod draw;

impl draw::Cursor {
    pub fn draw_square(&mut self, coords: [[u16;2];2], colour: Option<colours::RGB>, fx: Option<colours::FX>, text: String) {
        if colour.is_some() {
             self.print(format!("{}", colour.unwrap())).unwrap();
        }
        if fx.is_some() {
            self.print(format!("{}", fx.unwrap())).unwrap();
        }
        for y in coords[0][1]..coords[1][1] {
            for _ in coords[0][0]..coords[1][0] {
               self.print((&text).to_owned()).unwrap(); 
            }
            self.print("\n".to_string()).unwrap();
            self.goto(coords[0][0], y).unwrap();
        }
    }
}
