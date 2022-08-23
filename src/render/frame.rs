use crate::{ROWS, COLUMS};


pub type Frame = Vec<Vec<String>>;

pub fn new_frame(rows_were_start_strigs: Option<u8>) -> Frame {
    let mut cols = Vec::with_capacity(COLUMS.into());
    for _ in 0..=COLUMS {
        let mut col =Vec::with_capacity((ROWS as u8).into());
        let rows = match rows_were_start_strigs {
            None => {ROWS},
            Some(y) => {y - 1},
        };
        for _ in 0..=(rows as u8) {
            col.push("\x1b[1;39;40m ".to_string());
        }
        cols.push(col);
    }
    match rows_were_start_strigs {
        None => {return cols;},
        Some(_x) => {
            cols[0].push("\x1b[1;39;40m ".to_string());
            return cols;
        },
    };
}

pub trait Draw {
    fn draw(&self, frame: &mut Frame);
}