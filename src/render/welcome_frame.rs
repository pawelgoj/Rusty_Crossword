use crate::render::frame::{Draw, Frame};
use crate::COLUMS;

pub struct Welcome {
    positions_of_pixels: Vec<(u8, u8)>, //(x,y)
    inscription: String

}

impl Welcome {
    pub fn new() -> Self {
        Self {
            positions_of_pixels: vec![(1,8),(1,9),(1,10),(1,11),(1,12),(2,8),(2,10),(3,8),(3,9),(3,11),(3,12),
            (5,8),(5,9),(5,10),(5,11),(5,12),(6,12),(7,8),(7,9),(7,10),(7,11),(7,12),
            (9,8),(9,9),(9,10),(9,12),(10,8),(10,10),(10,12),(11,8),(11,10),(11,11),(11,12),
            (13,8),(14,8),(14,9),(14,10),(14,11),(14,12),(15,8),
            (17,8),(17,9),(17,10),(18,10),(18,11),(18,12),(19,8),(19,9),(19,10)],
            inscription: "CROSSWORD".to_string(),
        }
    }
}

impl Draw for Welcome {
    fn draw(&self, frame: &mut Frame) {
        let position = ((COLUMS - self.inscription.len() as u8) / 2) as u8;
        let first = true;
        let mut i: usize = 0;
        for pixel in &self.positions_of_pixels {
            i += 1;
            let p = pixel.0 + 5;
            let mut out = String::new();
            let char = '█'.to_string();

            if first {
                out.push_str("\x1b[1;91m");
                out.push_str(&char);
            }
            frame[ p as usize][pixel.1 as usize] = out;
        }
            let mut spaces = String::new();
            for _ in 0..position {
                spaces.push(' ');
            }

            let v= self.inscription.clone();

            spaces.push_str(&v);

            frame[0][15] = spaces;
        
    }
}