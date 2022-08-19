use std::{io, time::Duration};
use std::error::Error;

use crossterm::cursor::Show;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{terminal::{self, EnterAlternateScreen}, ExecutableCommand, cursor::Hide};
use crossword::sqllite_conection::Questions;
use crossterm::event::{self, Event, KeyCode};
use clap::Parser;
use rusty_audio::Audio;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli{
    #[clap(short, long, action)]
    new_game: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let _cmd = Cli::parse();

    //audio of game 
    let mut audio = Audio::new();
    audio.add("start", "sounds/correct.mp3");
    audio.add("lose", "sounds/lose.mp3");
    audio.add("win", "sounds/strat.mp3");
    audio.add("correct", "sounds/win.mp3");

    audio.play("start");

    //terminal 
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?; //włączmy Alternate Screan, ekran w cmd taki jak np. vim.
    stdout.execute(Hide)?; //ukrywany kursor.

    'loopgame: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'loopgame
                    }
                    _ => ()
                }
            }
        }
    }

    

    let mut questions = Questions::new();
    questions.load_questions_from_db("Password.db");

    for question in questions.questions {
        println!("{} {} {}", question.id, question.question, question.answer);
    }

    audio.wait();
    stdout.execute(Show)?; //pokazuje kursor 
    stdout.execute(LeaveAlternateScreen)?; //wychodzi z Alternate Screen
    terminal::disable_raw_mode()?; //wyłacza raw mode 

    Ok(())

}
