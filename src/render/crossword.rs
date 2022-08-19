use std::collections::HashSet;

use crate::{sqllite_conection::Question, COLUMS, ROWS};

pub enum Orientation {
    horizontally,
    perpendicularly
}

pub struct CrosswordKeyword { 
    question: Question,
    coord_start_x: u8,
    coord_start_y: u8,
    len_of_keyword: u8,
    orientation: Orientation
}

pub struct Crossword {
    crossword_keywords: Vec<CrosswordKeyword>,
}

impl Crossword {
    fn check_char_is_the_same(inserted: &CrosswordKeyword, in_crossword: &CrosswordKeyword,
        position_of_char_ins: u8, position_of_char_in_cross: u8) -> bool {

        let char_inserted = inserted.question.answer
            .chars().nth(position_of_char_ins.into()).unwrap();
        let char_in_crossword = in_crossword.question.answer
            .chars().nth(position_of_char_in_cross.into()).unwrap();

        if char_inserted == char_in_crossword {
            return true;
        } else {
            return false;
        }
    }

    fn check_collision(inserted: CrosswordKeyword, in_crossword: CrosswordKeyword ) -> bool {

        match inserted.orientation  {
            Orientation::horizontally => {
                match in_crossword.orientation {
                    Orientation::horizontally => {
                        if inserted.coord_start_y == in_crossword.coord_start_y 
                        && (in_crossword.coord_start_x + in_crossword.len_of_keyword >= inserted.coord_start_x)
                        && (in_crossword.coord_start_x -1 <= inserted.coord_start_x)
                        {
                            return true;
                        } else {
                            return false;
                        }
                    },
                    Orientation::perpendicularly => {
                        if (in_crossword.coord_start_x < inserted.coord_start_x - 1 
                            || in_crossword.coord_start_x > inserted.coord_start_x + 1) 
                            && (in_crossword.coord_start_y < inserted.coord_start_y -1 
                            || in_crossword.coord_start_y < inserted.coord_start_y +1) {
                                return false;
                        } else if (in_crossword.coord_start_x >= inserted.coord_start_x
                            && in_crossword.coord_start_x <= inserted.coord_start_x) 
                            && (in_crossword.coord_start_y <= inserted.coord_start_y 
                            && in_crossword.coord_start_y >= inserted.coord_start_y) {
                                
                                let position_of_char_in_cross = in_crossword.coord_start_x 
                                    - inserted.coord_start_x;
                                let position_of_char_ins = inserted.coord_start_y - in_crossword.coord_start_y;
                                let the_same_char = Crossword::check_char_is_the_same(&inserted, 
                                    &in_crossword, position_of_char_ins, position_of_char_in_cross);

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
            Orientation::perpendicularly => {
                match in_crossword.orientation {
                    Orientation::horizontally => { 
                        let in_crossword_x_max = in_crossword.coord_start_x + in_crossword.len_of_keyword;
                        let inserted_y_max = inserted.coord_start_y + inserted.len_of_keyword;
                        if (in_crossword.coord_start_x - 1 <= inserted.coord_start_x)
                            && (in_crossword_x_max + 1 >= inserted.coord_start_x)
                            && (in_crossword.coord_start_y >= inserted.coord_start_y -1)
                            && (in_crossword.coord_start_y <= inserted_y_max +1) {
                                let position_of_char_in_cross = inserted.coord_start_x 
                                    - in_crossword.coord_start_x;
                                let position_of_char_ins = in_crossword.coord_start_y - inserted.coord_start_y;
                                let the_same_char = Crossword::check_char_is_the_same(&inserted, 
                                    &in_crossword, position_of_char_ins, position_of_char_in_cross);
                                if the_same_char {
                                    return false;
                                } else {
                                    return true;
                                }
                            } else {
                                return false;
                            }
                    },
                    Orientation::perpendicularly => {
                        let in_crossword_y_max = in_crossword.coord_start_y 
                            + in_crossword.len_of_keyword;
                        if inserted.coord_start_x == in_crossword.coord_start_x 
                            && inserted.coord_start_y >= in_crossword.coord_start_y -1
                            && inserted.coord_start_y <= in_crossword_y_max {
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
    fn determine_position(question: &String, colums: u8, rows: u8,  first: bool, 
        questions_taken: &Vec<CrosswordKeyword>) -> Result<(u8, u8), String> {
            let question_len = question.len();
            if first {
                if question_len > (colums as usize) - 2 {
                    panic!("The keyword to long to crrosword in x direction")
                } else if question_len > (rows as usize) - 2  {
                    panic!("The keyword to long to crrosword in y direction")
                } else {
                    return Ok((2,2));
                }
            } else {
                


                return Err("Not implement yet!!!!!".to_string());
            }
    }

    pub fn new(questions: Vec<Question>) -> Self {
        let mut crossword_keywords:Vec<CrosswordKeyword> = Vec::new();

        for question in questions {
        
            let x_y = Crossword::determine_position(&question.answer, 
                COLUMS as u8, ROWS as u8, true, &crossword_keywords);

                let crossword_keyword = CrosswordKeyword{
                    len_of_keyword: question.answer.len() as u8,
                    question: question,
                    coord_start_x: 1, 
                    coord_start_y: 1,
                    orientation: Orientation::horizontally,
                };
                crossword_keywords.push(crossword_keyword);
        }
        Self { crossword_keywords: crossword_keywords }
    }

}

#[cfg(test)]
mod Test_of_crossword {
    use std::collections::HashSet;
    use crate::sqllite_conection::Question;

    use super::{Crossword, CrosswordKeyword, Orientation};

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
            orientation: orientation_insert
        };

        let crrossword_key_in_crossword = CrosswordKeyword{
            question: question2,
            coord_start_x: start_point_in_cross.0,
            coord_start_y: start_point_in_cross.1,
            len_of_keyword: 4,
            orientation: orientation_in_crossword
        };

        (crrossword_key_insert, crrossword_key_in_crossword)

    }
    #[test] 
    fn test_determine_position_first_position() {
        //Given 
        let tuple_keywords = test_mocks((1, 2), 
            Orientation::horizontally, (1,2), 
            Orientation::horizontally);
        let question = tuple_keywords.0.question.question;
        let mut positions_taken:Vec<CrosswordKeyword> = Vec::new();

        //When 
        let position = Crossword::determine_position(&question, 
            20, 20,true, &positions_taken);

        //Then 
        assert_eq!(position.unwrap(), (1 as u8, 1 as u8));
    }

    #[test]
    fn test_check_collision_no_collision() {
        //Given, horizontally, no collision
        let tuple_keywords = test_mocks((1, 2), 
            Orientation::horizontally, (1, 3), 
            Orientation::horizontally);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(!result);

        //Given, horizontally, no collision
        let tuple_keywords = test_mocks((1, 1), 
                Orientation::horizontally, (7, 1), 
                Orientation::horizontally);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(!result);

        //Given, perpendicularly, no collision
        let tuple_keywords = test_mocks((1, 1), 
        Orientation::perpendicularly, (2, 1), 
        Orientation::perpendicularly);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(!result);

        //Given, perpendicularly, no collision
        let tuple_keywords = test_mocks((1, 1), 
        Orientation::perpendicularly, (1, 7), 
        Orientation::perpendicularly);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(!result);

        //Given perdicuraly and horizontally
        let tuple_keywords = test_mocks((1, 1), 
        Orientation::horizontally, (3, 3), 
        Orientation::perpendicularly);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(!result);

        //Given perdicuraly and horizontally
        let tuple_keywords = test_mocks((1, 1), 
        Orientation::perpendicularly, (3, 1), 
        Orientation::horizontally);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(!result);

        //Given perdicuraly and horizontally
        let tuple_keywords = test_mocks((1, 1), 
        Orientation::horizontally, (2, 1), 
        Orientation::perpendicularly);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(!result);

        //Given perdicuraly and horizontally
        let tuple_keywords = test_mocks((5, 5), 
        Orientation::horizontally, (1, 1), 
        Orientation::perpendicularly);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(!result);

    }

    #[test]
    fn test_check_collision_collision() {
        //Given, horizontally, collision
        let tuple_keywords = test_mocks((1, 2), 
        Orientation::horizontally, (1, 2), 
        Orientation::horizontally);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(result);

        //Given, horizontally, collision
        let tuple_keywords = test_mocks((1, 1), 
        Orientation::horizontally, (6, 1), 
        Orientation::horizontally);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(result);

        //Given, perpendicularly, collision
        let tuple_keywords = test_mocks((1, 2), 
        Orientation::perpendicularly, (1, 2), 
        Orientation::perpendicularly);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(result);

        //Given, perpendicularly, collision
        let tuple_keywords = test_mocks((1, 1), 
        Orientation::perpendicularly, (1, 6), 
        Orientation::perpendicularly);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(result);

        //Given perdicuraly and horizontally
        let tuple_keywords = test_mocks((1, 1), 
        Orientation::horizontally, (1, 1), 
        Orientation::perpendicularly);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(result);

        //Given perdicuraly and horizontally
        let tuple_keywords = test_mocks((1, 1), 
        Orientation::perpendicularly, (1, 1), 
        Orientation::horizontally);
        //When 
        let result = Crossword::check_collision(tuple_keywords.1, tuple_keywords.0);
        assert!(result);

    }

    #[test]
    fn test_check_char() {
        //Given 
        //The same char 
        let tuple_keywords = test_mocks((1, 2), 
            Orientation::horizontally, (1,2), 
            Orientation::horizontally);
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