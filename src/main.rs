use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::{io, time::Duration};
use std::error::Error;

use crossterm::cursor::Show;
use crossterm::terminal::{LeaveAlternateScreen, Clear, ClearType};
use crossterm::{terminal::{self, EnterAlternateScreen}, ExecutableCommand, cursor::Hide};
use crossword::respose_to_user_and_instructions::{Instructions, ResponseToUser};
use crossword::{WINDOW_SIZE_X, WINDOW_SIZE_Y};
use crossword::render::crossword::Crossword;
use crossword::render::frame::{Draw, Frame, new_frame};
use crossword::render::render::render;
use crossword::render::welcome_frame::Welcome;
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

pub struct Message {
    stop_working: bool,
    frame: Option<Frame>
}
    fn break_game_loop(audio: &mut Audio, render_tx: &Sender<Message>) {
        audio.play("lose");
        let message = Message{stop_working: true, frame: None};
        let _ = render_tx.send(message);
    }



fn main() -> Result<(), Box<dyn Error>> {
    let _cmd = Cli::parse();

    //audio of game 
    let mut audio = Audio::new();
    audio.add("start", "sounds/correct.mp3");
    audio.add("lose", "sounds/lose.mp3");
    audio.add("win", "sounds/strat.mp3");
    audio.add("correct", "sounds/win.mp3");

    match enable_ansi_support::enable_ansi_support() {
        Ok(()) => {
            audio.play("correct")}
        Err(_) => {println!("Non supported!!")}
        }

    audio.play("start");

    //terminal 
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?; //Alternate Screen, screen like vim,
    stdout.execute(Hide)?; //Hide cursor.
    let _result = stdout.execute(
        terminal::SetSize(WINDOW_SIZE_X as u16, WINDOW_SIZE_Y as u16));

    //Render loop 
    let(render_tx, render_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    let render_handle = thread::spawn(move || {
        let mut frame_old  = new_frame(None);
        let mut frame_new = new_frame(Some(15));
        let welcome_screen = Welcome::new();
        let mut stdout = io::stdout();
        welcome_screen.draw(&mut frame_new);
        render(&mut stdout, &frame_old, &frame_new, false, true);
        let mut message_received = false;
        loop {
            let message = render_rx.recv().unwrap();
            match message.stop_working {
                true => {break},
                false => {
                    match message.frame {
                        Some(frame) => {
                            frame_new = frame; message_received = true;
                            render(&mut stdout, &frame_old, &frame_new, 
                                false, false);
                        },
                        None => {
                            stdout.execute(Hide).unwrap();
                            render(&mut stdout, &frame_old, &frame_new, false,
                                true); message_received = false;
                        }
                    }
                }
            }
            if message_received {
                frame_old = frame_new.clone();
            }
        }
    });

    //prepare questions from db.
    let mut questions = Questions::new();
    questions.load_questions_from_db("Password.db");
    questions.draw_questions_order();
    let mut crossword = Crossword::new(questions.return_questions(9));

    while crossword.crossword_keywords.len() > 4 {
        questions.draw_questions_order();
        crossword = Crossword::new(questions.return_questions(9));
    }

    thread::sleep(Duration::from_millis(1000));

    let start_strings: u8 = 22;
    let mut frame_new = new_frame(Some(start_strings));
    crossword.draw(&mut frame_new, start_strings);

    //Render crossword 
    let message = Message{stop_working: false, frame: Some(frame_new.clone())};
    let _ = render_tx.send(message);

    let instructions = Instructions::new();
    let response_to_user = ResponseToUser::new();


    //main game actions loop
    let mut user_action = false;
    'loopgame: loop {
        crossword.set_instructions_to_user((instructions.chose_clue.clone(), 
            instructions.end_game.clone()));
        crossword.draw(&mut frame_new, start_strings);
        let message = Message{stop_working: false, frame: Some(frame_new.clone())};
        let _ = render_tx.send(message);

        //block weiting to event 
        let event = event::read()?;
        match event {
            Event::Key(ref key_event) => {
                match key_event.code {
                    KeyCode::Esc => { 
                        break_game_loop(&mut audio, &render_tx);
                        break 'loopgame
                    },
                    _ => { //Other key press 
                        crossword.response_to_user(response_to_user.clear.clone());
                        crossword.draw(&mut frame_new, start_strings);
                        let message = Message{stop_working: false, frame: Some(frame_new.clone())};
                        let _ = render_tx.send(message);

                        'keyword: for i in 1..=crossword.crossword_keywords.len() {
                            if key_event.code == KeyCode::Char(i.to_string().chars().nth(0).unwrap()) {

                                if crossword.check_keyword_was_guessed(i as u8) {
                                    crossword.response_to_user(response_to_user.clue_was_guessed.clone());
                                    crossword.draw(&mut frame_new, start_strings);
                                    let message = Message{stop_working: false, frame: Some(frame_new.clone())};
                                    let _ = render_tx.send(message);
                                    break 'keyword
                                }

                                crossword.set_instructions_to_user((instructions.check_answer.clone(),
                                    instructions.end_game.clone()));
                                crossword.draw(&mut frame_new, start_strings);
                                let message = Message{stop_working: false, frame: Some(frame_new.clone())};
                                let _ = render_tx.send(message);

                                'user_write_answer: loop {
                                    let v = event::read()?;
                                    match v {
                                        Event::Key(key) => {
                                            match key.code {
                                                KeyCode::Char(ch) => {
                                                    let to_long_user_input = crossword.add_user_input(ch, i);
                                                    if to_long_user_input {
                                                        crossword.clear_user_input(i);
                                                        crossword.response_to_user(response_to_user.to_long_answer.clone());
                                                        crossword.set_instructions_to_user((instructions.check_answer.clone(),
                                                            instructions.end_game.clone()));
                                                    }
                                                    crossword.draw(&mut frame_new, start_strings);
                                                    let message = Message{stop_working: false, frame: Some(frame_new.clone())};
                                                    let _ = render_tx.send(message);
                                                },
                                                KeyCode::Enter => {
                                                    let input_is_correct =crossword.check_user_input_is_correct(i);
                                                    if input_is_correct {
                                                        crossword.add_guessed_keyword(i as u8);
                                                        crossword.response_to_user(response_to_user.correct_answer.clone());
                                                    } else {
                                                        crossword.clear_user_input(i);
                                                        crossword.response_to_user(response_to_user.in_correct_answer.clone());                          
                                                    }
                                                    crossword.draw(&mut frame_new, start_strings);
                                                    let message = Message{stop_working: false, frame: Some(frame_new.clone())};
                                                    let _ = render_tx.send(message);

                                                    break 'user_write_answer
                                                },
                                                KeyCode::Esc => {
                                                    break_game_loop(&mut audio, &render_tx);
                                                    break 'loopgame
                                                },
                                                _ => {}
                                            }
                                        },
                                        _ => {}
                                    }
                                }
                                break 'keyword;
                                
                            } else { continue }
                        }
                    }
                }
            },
            Event::Resize(_colum, _row) => {
                if user_action {
                    user_action = false;
                    let _result = stdout.execute(
                        terminal::SetSize(WINDOW_SIZE_X as u16, WINDOW_SIZE_Y as u16));
                    stdout.execute(Clear(ClearType::All)).unwrap();
                    stdout.execute(Hide).unwrap();

                    let message = Message{stop_working: false, frame: None};
                    let _ = render_tx.send(message);
                } else {
                    user_action = true;
                }
            },
            _ => {}
        }

        if crossword.user_guessed_all_clues() {
            crossword.response_to_user(response_to_user.all_correct.clone());
            crossword.draw(&mut frame_new, start_strings);
            let message = Message{stop_working: false, frame: Some(frame_new.clone())};
            let _ = render_tx.send(message);

            thread::sleep(Duration::from_millis(2000));
            break 'loopgame
        }
    }



    //Cleanup
    let message = Message{stop_working: true, frame: None};
    let _ = render_tx.send(message);
    audio.wait();
    drop(render_tx);
    render_handle.join().unwrap();
    stdout.execute(Show)?; 
    stdout.execute(LeaveAlternateScreen)?; 
    terminal::disable_raw_mode()?; 

    Ok(())

}
