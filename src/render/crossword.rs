use std::collections::HashSet;

use crate::{sqllite_conection::Question, COLUMS, ROWS, WINDOW_SIZE_X};
use rand::Rng;
use super::frame::Frame;


#[derive(Debug)]
#[derive(Clone)]
pub enum Orientation {
    Horizontally,
    Perpendicularly
}


#[derive(Debug)]
#[derive(Clone)]
pub struct CrosswordKeyword { 
    question: Question,
    coord_start_x: u8,
    coord_start_y: u8,
    len_of_keyword: u8,
    orientation: Orientation,
    user_input: Option<String>
}

#[derive(Debug)]
pub struct Crossword {
    pub crossword_keywords: Vec<CrosswordKeyword>,
    guessed_clues : HashSet<u8>,
    response_to_user: Option<String>,
    instructions_to_user: Option<((String, u8), (String, u8))>
}

impl Crossword {
    fn arrange_indexes_in_order(crossword_keywords: Vec<CrosswordKeyword>) -> Vec<CrosswordKeyword> {
        let mut i: u8 = 0;
        let mut arranged = Vec::new();
        for mut keyword in crossword_keywords {
            i += 1;
            keyword.question.id = i as u64;
            arranged.push(keyword);
        }
        arranged
    }

    fn check_is_in_frame(x_min: i8, x_max: i8, y_min: i8, y_max: i8) -> bool {
        // Check is in frame, frame range COLUMS or ROWS 0..=VALUE. 
        if x_min  < 4 || y_min < 4 || x_max > (COLUMS - 3) as i8 || y_max > (ROWS -20) as i8 {
            return false; 
        } else {
            return true;
        }
    }

    fn find_position_of_keyword_by_intersect(mut question: CrosswordKeyword, question_in_crossword: &CrosswordKeyword, 
        questions_taken: &Vec<CrosswordKeyword>) -> Option<CrosswordKeyword> {
        //if questions have not intersect return None 

        let crossword_string = question.question.answer.clone();
        let crossword_string_2 = question_in_crossword.question.answer.clone();

        let mut i_in_crossword = 0;
        let mut index = None;

        for ch in crossword_string_2.chars() {
            index = crossword_string.find(ch);

            match index {
                Some(index) => { 
                    match question_in_crossword.orientation {
                        Orientation::Horizontally => {
                            question.orientation = Orientation::Perpendicularly;
                            let x_min = question_in_crossword.coord_start_x as i8 + i_in_crossword as i8;
                            let y_min = question_in_crossword.coord_start_y as i8 - index as i8;
                            let x_max = x_min;
                            let y_max = y_min + question.len_of_keyword as i8 -1;

                            if Crossword::check_is_in_frame(x_min, x_max, y_min, y_max) {
                                question.coord_start_x = x_min as u8;
                                question.coord_start_y = y_min as u8;

                                let mut collision = false; 
                                'check_collision: for question_taken in questions_taken {
                                    collision = Crossword::check_collision(&question, &question_taken);
                                    if collision {
                                        break 'check_collision
                                    }
                                }
                                if !collision {
                                    return Some(question);
                                }
                            }
                        },
                        Orientation::Perpendicularly => {
                            question.orientation = Orientation::Horizontally;

                            let x_min = question_in_crossword.coord_start_x as i8 - index as i8;
                            let y_min = question_in_crossword.coord_start_y as i8 + i_in_crossword as i8;
                            let x_max = x_min + question.len_of_keyword as i8 -1;
                            let y_max = y_min;

                            if Crossword::check_is_in_frame(x_min, x_max, y_min, y_max) {
                                question.coord_start_x = x_min as u8;
                                question.coord_start_y = y_min as u8;

                                let mut collision = false; 
                                'check_collision_2: for question_taken in questions_taken {
                                    collision = Crossword::check_collision(&question, &question_taken);
                                    if collision {
                                        break 'check_collision_2
                                    }
                                }
                                if !collision {
                                    return Some(question);
                                }
                            }
                        }
                    }
                },
                None => {}
            }

            i_in_crossword+=1;

        };

        return None;

    }

    fn check_char_is_the_same(inserted: &CrosswordKeyword, in_crossword: &CrosswordKeyword,
        position_of_char_ins: u8, position_of_char_in_cross: u8) -> bool {

        let char_inserted = inserted.question.answer
            .chars().nth(position_of_char_ins.into());
        match char_inserted {
            Some(_char_inserted) => {},
            None => {return false}
        }

        let char_in_crossword = in_crossword.question.answer
            .chars().nth(position_of_char_in_cross.into());
        match char_in_crossword {
            Some(_char_in_crossword) => {},
            None => {return false}
        }

        if char_inserted == char_in_crossword {
            return true;
        } else {
            return false;
        }
    }

    fn check_collision(inserted: &CrosswordKeyword, in_crossword: &CrosswordKeyword ) -> bool {

        match inserted.orientation  {
            Orientation::Horizontally => {
                match in_crossword.orientation {
                    Orientation::Horizontally => {
                        let inserted_x_max = inserted.coord_start_x + inserted.len_of_keyword;
                        let in_crossword_x_max = in_crossword.coord_start_x + in_crossword.len_of_keyword;
                        if (in_crossword.coord_start_y == inserted.coord_start_y)
                        && (((in_crossword_x_max +1 >= inserted.coord_start_x)
                        && (in_crossword.coord_start_x -2 <= inserted.coord_start_x))
                        || ((inserted_x_max +1 >= in_crossword.coord_start_x)
                        && (in_crossword.coord_start_x-2 <= in_crossword.coord_start_x)))
                        {
                            return true;
                        } else {
                            return false;
                        }
                    },
                    Orientation::Perpendicularly => {
                        let inserted_x_max = inserted.coord_start_x + inserted.len_of_keyword;
                        let in_crossword_y_max = in_crossword.coord_start_y + in_crossword.len_of_keyword;
                        if (in_crossword.coord_start_x >= inserted.coord_start_x - 2) 
                            && (in_crossword.coord_start_x <= inserted_x_max + 1)
                            && (inserted.coord_start_y >= in_crossword.coord_start_y -2) 
                            && (inserted.coord_start_y <= in_crossword_y_max + 1) {
                                
                                let position_of_char_in_cross = inserted.coord_start_y as i8
                                    - in_crossword.coord_start_y as i8;
                                let position_of_char_ins = in_crossword.coord_start_x as i8 
                                    - inserted.coord_start_x as i8;
                                
                                if position_of_char_in_cross < 0 || position_of_char_ins < 0 {
                                    return true;
                                }

                                let the_same_char = Crossword::check_char_is_the_same(&inserted, 
                                    &in_crossword, position_of_char_ins as u8, position_of_char_in_cross as u8);

                                if the_same_char {
                                    return false;
                                } else {
                                    return true;
                                }
                        } else {
                            return true;
                        }
                    }
                }
            },
            Orientation::Perpendicularly => {
                match in_crossword.orientation {
                    Orientation::Horizontally => { 
                        let in_crossword_x_max = in_crossword.coord_start_x + in_crossword.len_of_keyword;
                        let inserted_y_max = inserted.coord_start_y + inserted.len_of_keyword;
                        if (in_crossword.coord_start_x - 2 <= inserted.coord_start_x)
                            && (in_crossword_x_max + 1 >= inserted.coord_start_x)
                            && (in_crossword.coord_start_y >= inserted.coord_start_y -2)
                            && (in_crossword.coord_start_y <= inserted_y_max +1) {

                                let position_of_char_in_cross = inserted.coord_start_x as i8 
                                    - in_crossword.coord_start_x as i8;

                                let position_of_char_ins = in_crossword.coord_start_y as i8 
                                    - inserted.coord_start_y as i8;
                                
                                if position_of_char_in_cross < 0 || position_of_char_ins < 0 {
                                    return true;
                                }

                                let the_same_char = Crossword::check_char_is_the_same(&inserted, 
                                    &in_crossword, position_of_char_ins as u8, position_of_char_in_cross as u8);

                                if the_same_char {
                                    return false;
                                } else {
                                    return true;
                                }
                            } else {
                                return false;
                            }
                    },
                    Orientation::Perpendicularly => {
                        let in_crossword_y_max = in_crossword.coord_start_y 
                            + in_crossword.len_of_keyword;
                        let inserted_y_max = inserted.coord_start_y + inserted.len_of_keyword;
                        if (in_crossword.coord_start_x == inserted.coord_start_x)
                            && (((in_crossword.coord_start_y -2 <= inserted.coord_start_y)
                            && (in_crossword_y_max + 1 >=  inserted.coord_start_y))
                            || ((inserted.coord_start_y -2 <= in_crossword.coord_start_y)
                            && (inserted_y_max +1 >= in_crossword.coord_start_y))) {
                            return true;
                        } else {
                            return false;
                        }
                    }
                }
            }
        } 
    }
    
    //return coord_start_x and coord_start_y
    fn determine_position(mut question: CrosswordKeyword, first: bool, 
        questions_taken: &Vec<CrosswordKeyword>) -> Option<CrosswordKeyword> {
        let colums= COLUMS -6;
        let rows = ROWS -20;
            
        let question_len = question.question.answer.len();

        if question_len > (colums as usize) - 2 {
            panic!("The keyword to long to crrosword in x direction")
        } else if question_len > (rows as usize) - 2  {
            panic!("The keyword to long to crrosword in y direction")
        } else {
            if first {
                let mut rng = rand::thread_rng();
                let pos = rng.gen_range(5..=10);
                question.coord_start_x = pos as u8;
                question.coord_start_y = pos as u8;
                return Some(question);
            } else {
                for question_in_crossword in questions_taken {
                    let question_tym = Crossword::find_position_of_keyword_by_intersect(
                        question.clone(), &question_in_crossword, &questions_taken);
                    match question_tym {
                        None => {}, 
                        Some(question_tym) => {
                            return Some(question_tym);
                        }
                    }
                }
                return None;
            }
        }
    }

    pub fn check_keyword_is_in_crossword(&self, id: usize) -> bool {
        for keyword in &self.crossword_keywords {
            if keyword.question.id as usize == id {
                return true;
            }
        }
        return false 
    }

    pub fn new(questions: Vec<Question>) -> Self {
        let mut questions_taken:Vec<CrosswordKeyword> = Vec::new();
        let mut first = true;

        for question in questions {

            let crossword_keyword = CrosswordKeyword{
                len_of_keyword: question.answer.len() as u8,
                question: question,
                coord_start_x: 1, 
                coord_start_y: 1,
                orientation: Orientation::Horizontally, //work horizontally and perdicirally
                user_input: None
            };

            let crossword_keyword = Crossword::determine_position(crossword_keyword, 
                first, &questions_taken);

            match crossword_keyword {
                None => {},
                Some(crossword_keyword) => {questions_taken.push(crossword_keyword);}
            }
            first = false;
        }
        let questions_taken = Crossword::arrange_indexes_in_order(questions_taken);
        let mut guessed_clues = HashSet::new();

        Self { crossword_keywords: questions_taken, response_to_user: None, instructions_to_user: None, 
            guessed_clues: guessed_clues}
    }

    pub fn response_to_user(&mut self, message: String) {
        self.response_to_user = Some(message);
    }

    pub fn add_user_input(&mut self, char: char, question_id: usize) -> bool {
        for keyword in &mut self.crossword_keywords {
            if keyword.question.id == question_id as u64 {
                match keyword.user_input {
                    Some(ref mut input) => {input.push_str(&char.to_string());},
                    None => {keyword.user_input = Some(char.to_string());}
                }

                let user_input_len = match keyword.user_input {
                    Some(ref x) => {x.len()},
                    None => {0}
                };

                if keyword.question.answer.len() >= user_input_len {
                    return false;
                } else {
                    return true;
                }
            }
            
        }
        return false;
    }

    pub fn clear_user_input(&mut self, question_id: usize) {
        for keyword in &mut self.crossword_keywords {
            if keyword.question.id == question_id as u64 {
                keyword.user_input = None;
                break;
            }
        }
    }

    pub fn set_instructions_to_user(&mut self, instructions: ((String, u8), (String, u8))) {
        if instructions.0.0.len() > (60 + instructions.0.1 as usize)
            || instructions.1.0.len() > (60 + instructions.1.1 as usize) {
            panic!("To long instruction to user!!!!!!!");
        }
        self.instructions_to_user = Some(instructions);
    }

    pub fn check_user_input_is_correct(&mut self, id: usize ) -> bool {
        for keyword in &self.crossword_keywords {
            if keyword.question.id == id as u64 {
                match keyword.user_input {
                    Some(ref user_input) => {
                        if user_input.to_uppercase() == keyword.question.answer.to_uppercase() {
                            return true;
                        } else {
                            return false;
                        }
                    },
                    None => {return false;}
                }
            }
        }
        return false;
    }

    pub fn check_keyword_was_guessed(&self, id: u8) -> bool {
        let contains = self.guessed_clues.contains(&id);
        contains
    }

    pub fn add_guessed_keyword(&mut self, id: u8) {
        self.guessed_clues.insert(id);
    }

    pub fn user_guessed_all_clues(&self) -> bool {
        let clues = self.crossword_keywords.clone();
        for clue in clues {
            if !self.guessed_clues.contains(&(clue.question.id as u8)) {
                return false;
            }
        }
        return true;
    }
}

impl Crossword {

    pub fn draw(&self, frame: &mut super::frame::Frame, strings_start: u8) {
        let char = "\x1b[1;37m▒";
        let crossword_keywords = &self.crossword_keywords;
        let mut questions_out = String::new();

        let mut positions_with_leters: HashSet<(u8, u8)> = HashSet::new();

        for key_word in crossword_keywords { 
            let col = key_word.coord_start_x;
            let row = key_word.coord_start_y;
            let mut user_input = String::new();
            if key_word.user_input == None {

            } else {
                user_input = key_word.user_input.clone().unwrap();
            }
            for i in 0..key_word.len_of_keyword {
                insert_char_in_crossword(&mut positions_with_leters, 
                    &key_word.orientation, i, col, row, frame, key_word, char, &user_input);
 
            }
            questions_out.push_str(&key_word.question.id.to_string());
            questions_out.push('.');
            questions_out.push(' ');
            questions_out.push_str(&key_word.question.question);
            questions_out.push_str(&"\n");
        }
        match &self.response_to_user {
            Some(ref response ) => {questions_out.push_str(&response);
                questions_out.push_str(&"\n");},
            None => {}
        }
        
        //instructions:
        let mut output = String::new();

        let instructions =  match self.instructions_to_user {
            Some(ref instruction) => {

            let (line_1, line_2) = instruction.clone();


            let mut line_1 = complete_with_spaces(line_1.0, 
                (WINDOW_SIZE_X + line_1.1) as usize);
            let line_2 = complete_with_spaces(line_2.0, 
                (WINDOW_SIZE_X + line_2.1) as usize);

            line_1.push_str(&line_2);
            line_1
        },
            None => {"\n".to_string()}
        };

        output.push_str(&instructions);
        output.push_str(&" \n\n");
        output.push_str(&questions_out);
        frame[0][strings_start as usize] = output;



        fn complete_with_spaces(mut text: String, lenght: usize) -> String {
            let number_of_space = lenght - text.len();
            for _ in 0..number_of_space {
                text.push_str(&" ".to_string());
            }
            text
        }

        fn insert_char_in_crossword (positions_with_leters: &mut HashSet<(u8, u8)>, orientation: &Orientation, i: u8, 
            mut col: u8, mut row: u8, frame: &mut Frame, key_word: &CrosswordKeyword, char: &str, user_input: &String) {
            
            let col_id;
            let col_arrow;
            let row_id;
            let row_arrow;
            let arrow;

            match orientation {
                Orientation::Horizontally => {
                    col_id = col -2;
                    col_arrow = col -1;
                    row_id = row;
                    row_arrow = row;
                    col = col + i;
                    arrow = "\x1b[1;36;40m>";
                },

                Orientation::Perpendicularly => {
                    col_id = col;
                    col_arrow = col;
                    row_id = row -2;
                    row_arrow = row -1;
                    row = row + i;
                    arrow = "\x1b[1;36;40mv";
                }
            }
            
            if i == 0 {
                let mut out = "\x1b[1;31;40m".to_string();
                out.push_str(&key_word.question.id.to_string());
                frame[(col_id) as usize][row_id as usize] = out;
                frame[(col_arrow) as usize][row_arrow as usize] = arrow.to_string();
            } 
            if key_word.user_input == None && !positions_with_leters.contains(&((col) as u8, row as u8)) {
                frame[(col) as usize][(row) as usize] = char.to_string();
            } else {
                if !positions_with_leters.contains(&((col) as u8, row as u8)) {
                    if i < user_input.len() as u8 {
                        frame[(col) as usize][(row) as usize] = user_input.chars()
                            .nth(i.into()).unwrap().to_string();
                        positions_with_leters.insert(((col) as u8, row as u8));

                    } else {

                        frame[(col) as usize][(row) as usize] = char.to_string();

                    }
                }
            }
        }
    }

}



/*
Unit tests 
-------------------------------------------------------------------------------------------------
 */
#[cfg(test)]
mod test_of_crossword {
    use crate::sqllite_conection::{Question, Questions};

    use super::{Crossword, CrosswordKeyword, Orientation};

    struct PreConditions { 
        path: String,
    }

    impl PreConditions {
        pub fn new() -> Self {
            Self {path: String::from("Password.db")}
        }
    }

    fn test_mocks(start_point_insert: (u8, u8), orientation_insert: Orientation,
        start_point_in_cross: (u8, u8), orientation_in_crossword: Orientation) -> 
        (CrosswordKeyword, CrosswordKeyword) {

        let question1 = Question{
            id: 1, 
            question: "Test question?".to_string(),
            answer: "kamil".to_string()
        };

        let question2 = Question{
            id: 2, 
            question: "Test question?".to_string(),
            answer: "adam".to_string()
        };

        let crrossword_key_insert = CrosswordKeyword{
            question: question1,
            coord_start_x: start_point_insert.0,
            coord_start_y: start_point_insert.1,
            len_of_keyword: 5,
            orientation: orientation_insert,
            user_input: None
        };

        let crrossword_key_in_crossword = CrosswordKeyword{
            question: question2,
            coord_start_x: start_point_in_cross.0,
            coord_start_y: start_point_in_cross.1,
            len_of_keyword: 4,
            orientation: orientation_in_crossword,
            user_input: None
        };

        (crrossword_key_insert, crrossword_key_in_crossword)

    }

    #[test] 
    fn test_check_is_in_frame() {
        //Max crossword canva
        let result = Crossword::check_is_in_frame(4, 27, 4, 20);
        assert!(result);

        let result = Crossword::check_is_in_frame(-1, 5, 4, 9);
        assert!(!result);
    }

    #[test]
    fn test_new_crossword() {
        //Given 
        let pre_conditions = PreConditions::new();
        let path_to_db = pre_conditions.path.as_str();
        let mut questions = Questions::new();
        //When 
        questions.load_questions_from_db(path_to_db);
        questions.load_questions_from_db(path_to_db);
        questions.draw_questions_order();
        //Then
        let vec_questions = questions.return_questions(7);
        assert_eq!(vec_questions.len(), 7);
        let crossword = Crossword::new(vec_questions);

        println!("{:?}", crossword.crossword_keywords);
        println!("keywords: {:?}", crossword.crossword_keywords.len());
        assert!(true);

    }

    #[test]
    fn test_find_position_of_keyword_by_intersect() {
        //Given no other position taken, positive test case 
        let positions_taken:Vec<CrosswordKeyword> = Vec::new();
        //paddings have 4 point
        let mocks = test_mocks((5,5), 
            Orientation::Horizontally, (4,4), 
            Orientation::Horizontally);

        //When
        let result = Crossword::find_position_of_keyword_by_intersect(
            mocks.1, &mocks.0, &positions_taken);
        let result = result.unwrap();

        //Then
        let mut result_orientation = false;

        match result.orientation {
            Orientation::Perpendicularly => {result_orientation = true},
            Orientation::Horizontally => {result_orientation = false}
        };

        assert!(result_orientation);
        assert_eq!(result.coord_start_x, 6);
        assert_eq!(result.coord_start_y, 5);

        //Given no other position taken, positive test case 
        let positions_taken:Vec<CrosswordKeyword> = Vec::new();
        let mocks = test_mocks((5,5), 
            Orientation::Perpendicularly, (4,4), 
            Orientation::Perpendicularly);

        //When
        let result = Crossword::find_position_of_keyword_by_intersect(
            mocks.1, &mocks.0, &positions_taken);
        let result = result.unwrap();

        //Then
        let mut result_orientation = false;

        match result.orientation {
            Orientation::Horizontally => {result_orientation = true},
            Orientation::Perpendicularly => {result_orientation = false}
        };

        assert!(result_orientation);
        assert_eq!(result.coord_start_x, 5);
        assert_eq!(result.coord_start_y, 6);


        //Given, Negative test case, expected to return None 
        let question_to_negative_test_case = Question{
            id: 1, 
            question: "Test question?".to_string(),
            answer: "XXXXXX".to_string()
        };

        let crrossword_key_insert = CrosswordKeyword{
            question: question_to_negative_test_case,
            coord_start_x: 1,
            coord_start_y: 1,
            len_of_keyword: 5,
            orientation: Orientation::Horizontally,
            user_input: None
        };

        let positions_taken:Vec<CrosswordKeyword> = Vec::new();
        let mocks = test_mocks((5,5), 
            Orientation::Horizontally, (4,4), 
            Orientation::Horizontally);

        //When
        let result = Crossword::find_position_of_keyword_by_intersect(
            mocks.1, &crrossword_key_insert, &positions_taken);

        //Then
        let result = match result {
            Some(_result) => false,
            None => true
            
        };
        assert!(result);
    }
    
    #[test]
    fn test_check_collision_no_collision() {
        //Given, horizontally, no collision
        let tuple_keywords = test_mocks((4, 5), 
            Orientation::Horizontally, (4, 6), 
            Orientation::Horizontally);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(!result);

        //Given, horizontally, no collision
        let tuple_keywords = test_mocks((4, 4), 
                Orientation::Horizontally, (10, 4), 
                Orientation::Horizontally);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(!result);

        //Given, perpendicularly, no collision
        let tuple_keywords = test_mocks((4, 4), 
        Orientation::Perpendicularly, (5, 4), 
        Orientation::Perpendicularly);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(!result);

        //Given, perpendicularly, no collision
        let tuple_keywords = test_mocks((4, 4), 
        Orientation::Perpendicularly, (4, 10), 
        Orientation::Perpendicularly);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(!result);

        //Given perdicuraly and horizontally
        let tuple_keywords = test_mocks((4, 4), 
        Orientation::Horizontally, (6, 6), 
        Orientation::Perpendicularly);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(!result);

        //Given perdicuraly and horizontally
        let tuple_keywords = test_mocks((4, 4), 
        Orientation::Perpendicularly, (6, 4), 
        Orientation::Horizontally);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(!result);

        //Given perdicuraly and horizontally
        let tuple_keywords = test_mocks((4, 4), 
        Orientation::Horizontally, (5, 4), 
        Orientation::Perpendicularly);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(!result);

        //Given perdicuraly and horizontally
        let tuple_keywords = test_mocks((8, 8), 
        Orientation::Horizontally, (4, 4), 
        Orientation::Perpendicularly);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(!result);

    }

    #[test]
    fn test_check_collision_collision() {
        //Given, horizontally, collision
        let tuple_keywords = test_mocks((4, 5), 
        Orientation::Horizontally, (4, 5), 
        Orientation::Horizontally);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(result);

        //Given, horizontally, collision
        let tuple_keywords = test_mocks((4, 4), 
        Orientation::Horizontally, (9, 4), 
        Orientation::Horizontally);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(result);

        //Given, perpendicularly, collision
        let tuple_keywords = test_mocks((4, 5), 
        Orientation::Perpendicularly, (4, 5), 
        Orientation::Perpendicularly);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(result);

        //Given, perpendicularly, collision
        let tuple_keywords = test_mocks((4, 4), 
        Orientation::Perpendicularly, (4, 9), 
        Orientation::Perpendicularly);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(result);

        //Given perdicuraly and horizontally
        let tuple_keywords = test_mocks((4, 4), 
        Orientation::Horizontally, (4, 4), 
        Orientation::Perpendicularly);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(result);

        //Given perdicuraly and horizontally
        let tuple_keywords = test_mocks((4, 4), 
        Orientation::Perpendicularly, (4, 4), 
        Orientation::Horizontally);
        //When 
        let result = Crossword::check_collision(&tuple_keywords.1, &tuple_keywords.0);
        assert!(result);

    }

    #[test]
    fn test_check_char() {
        //Given 
        //The same char 
        let tuple_keywords = test_mocks((4, 5), 
            Orientation::Horizontally, (4,5), 
            Orientation::Horizontally);
        //When
        let result = super::Crossword::check_char_is_the_same(&tuple_keywords.0, 
            &tuple_keywords.1, 1, 2);
        //Then
        assert!(result);
        //Wen other chart
        let result = super::Crossword::check_char_is_the_same(&tuple_keywords.0, 
            &tuple_keywords.1, 2, 2);
        assert!(!result);

    }
}