pub struct Instructions {
    pub chose_clue: (String, u8), //string and non printable chars
    pub end_game: (String, u8),
    pub check_answer: (String, u8) 
}
 pub struct ResponseToUser {
    pub to_long_answer: String,
    pub correct_answer: String,
    pub in_correct_answer: String
}

impl Instructions {
    pub fn new() -> Self {
        Self {
            chose_clue: ("press the crossword clue number, e.g. \x1b[1;32m'1' \x1b[1;37m".to_string(), 14),
            end_game: ("\x1b[1;37mpress \x1b[1;31m'esc' \x1b[1;37mto quit game".to_string(), 21),
            check_answer: ("Write answer and press \x1b[1;32m'Enter' \x1b[1;37m".to_string(), 14)
        }
    }
}

impl ResponseToUser {
    pub fn new() -> Self {
    Self {
            to_long_answer: "\x1b[1;93mKeyword was to long!!!!!!! Write again.\x1b[1;37m".to_string(),
            correct_answer: "\x1b[1;92mCorrect answer!!!!!!!\x1b[1;37m                       ".to_string(),
            in_correct_answer: "\x1b[1;91mIncorrect answer!!!!!!! Write again.\x1b[1;37m     ".to_string()
        }
    }
}





