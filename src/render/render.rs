use std::io::{Stdout, Write};

use crossterm::{cursor::MoveTo, QueueableCommand, style::{SetBackgroundColor, Color}, terminal::{Clear, ClearType}};

use super::frame::Frame;




//reder something in frame
pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, change_color: bool, force: bool) {
    if change_color {
        stdout.queue(SetBackgroundColor(Color::Green)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap(); 

    }
    
    //this render what is in the frame.
    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap(); //position of cursor 
                print!("{}", *s);
            }
        }
    }
    //execute the commands in queue
    stdout.flush().unwrap();
}