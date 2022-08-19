use crate::{ROWS, COLUMS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn clear_frame() -> Frame {
    let mut cols = Vec::with_capacity(COLUMS.into());
    for _ in 0..COLUMS {
        let mut col =Vec::with_capacity(ROWS.into());
        for _ in 0..ROWS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

pub trait Draw {
    fn draw(&self, frame: Frame);
}