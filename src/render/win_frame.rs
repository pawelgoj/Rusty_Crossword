use crate::render::frame::{Draw, Frame};
use crate::COLUMS;

pub struct Win {
    positions_of_pixels: Vec<(u8, u8)>, //(x,y)

}

impl Win {
    pub fn new() -> Self {
        Self {
            positions_of_pixels: vec![(1,8),(1,9),(1,10),(1,11),(1,12),(2,12),(3,12),(3,11),(3,10),
                (4,12),(5,12),(5,11),(5,10),(5,9),(5,8),
                (7,12),(7,8),(8,8),(8,9),(8,10),(8,11),(8,12),(9,12),(9,8),
                (11,8),(11,9),(11,10),(11,11),(11,12),(12,8),(13,8),(14,8),(14,9),(14,10),(14,11),(14,12)
                ,(16,8),(16,9),(16,10),(16,12)
                ,(18,8),(18,9),(18,10),(18,12)
                ,(20,8),(20,9),(20,10),(20,12)
            ],
        }
    }
}

impl Draw for Win {
    fn draw(&self, frame: &mut Frame) {
        let first = true;
        let mut i: usize = 0;
        for pixel in &self.positions_of_pixels {
            i += 1;
            let p = pixel.0 + 5;
            let mut out = String::new();
            let char = '█'.to_string();

            if first {
                out.push_str("\x1b[1;92m");
                out.push_str(&char);
            }
            frame[p as usize][pixel.1 as usize] = out;
        }
            let mut spaces = String::new();
            for _ in 0..COLUMS {
                spaces.push(' ');
            }
    }
}