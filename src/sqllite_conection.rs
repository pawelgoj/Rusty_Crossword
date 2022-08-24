use sqlite::State;
use rand::Rng;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[derive(Clone)]
pub struct Question {
    pub id: u64,
    pub question: String,
    pub answer: String,
}
#[derive(Debug)]
#[derive(Clone)]
pub struct Questions {
    pub questions: Vec<Question>,
    pub len: u8,
}




impl Questions {

    pub fn new() -> Self {
        let question = Question {
            id: 0,
            question: String::new(),
            answer: String::new(),
        };
        Self { 
            questions: vec![question],
            len: 0,
        }
    }

    pub fn load_questions_from_db(&mut self, path: &str) {

        let conn = sqlite::open(path).unwrap();
        let mut statement = conn.prepare("SELECT * FROM Questions",).unwrap();

        self.questions.clear();

        while let State::Row = statement.next().unwrap() {
            let id = statement.read::<i64>(0).unwrap();
            let question = statement.read::<String>(1).unwrap();
            let answer = statement.read::<String>(2).unwrap().to_uppercase();

            let question = Question{
                id: id as u64,
                question: question,
                answer: answer,
            };
            self.questions.push(question);
        }
        self.len = self.questions.len() as u8;
    }

    pub fn draw_questions_order(&mut self) {
        let mut rng = rand::thread_rng();
        let mut list_of_used_ids = Vec::new();
        let len = self.len;
        for guestion in &mut self.questions {

            'question: loop {
                let ids = rng.gen_range(1..=len);

                if !list_of_used_ids.contains(&[ids]) {
                    guestion.id = ids as u64;
                    list_of_used_ids.push([ids]);
                    break 'question;

                } else if list_of_used_ids.len() < len as usize {
                    continue
                } else {
                    panic!("Range of loop is incorrect!!!")
                }
            }
        }
    }
    
    pub fn return_questions(&self, number_of_questions: u8) -> Vec<Question> {
        let mut questions = self.questions.clone();
        questions.sort_by(|a, b| a.id.cmp(&b.id));
        let questions = questions[0..number_of_questions as usize].to_vec();
        questions
    }
    

}

#[cfg(test)]
mod test_of_questions {

    struct PreConditions { 
        path: String,
    }

    impl PreConditions {
        pub fn new() -> Self {
            Self {path: String::from("Password.db")}
        }
    }

    #[test]
    fn test_correct_len() {
        //Given
        let pre_conditions = PreConditions::new();
        let path_to_db = pre_conditions.path.as_str();
        let actual_len = 0; 
        //When empty 
        let mut questions = super::Questions::new();
        //Then 
        assert_eq!(questions.len, actual_len);
        //When loaded data from data base 
        let actual_len = 8;
        questions.load_questions_from_db(path_to_db);
        //Then 
        assert_eq!(questions.len, actual_len);

    }

    #[test]
    fn draw_question_order() {
        //Given 
        let pre_conditions = PreConditions::new();
        let path_to_db = pre_conditions.path.as_str();
        let mut question = super::Questions::new();
        //When 
        question.load_questions_from_db(path_to_db);
        let previous_questions = question.clone();
        question.draw_questions_order();
        //Then 

        let mut questions_vec = Vec::new();
        for item in question.questions {
            questions_vec.push(item.id);
        }
        let mut questions_vec_pre = Vec::new();
        for item in previous_questions.questions {
            questions_vec_pre.push(item.id);
        }
        
        //check allowed id >0
        let mut correct = true;
        
        for item in &questions_vec {
            if *item < 1 {
                correct = false;
            }
        }
        assert_ne!(questions_vec_pre, questions_vec);
        assert!(correct);
        println!("pre: {:?}", questions_vec_pre);
        println!("draw: {:?}", questions_vec);
    }

    #[test]
    fn test_o_vec() {
        //Given
        let pre_conditions = PreConditions::new();
        let path_to_db = pre_conditions.path.as_str();
        let mut questions = super::Questions::new();
        //When 
        questions.load_questions_from_db(path_to_db);
        questions.load_questions_from_db(path_to_db);
        questions.draw_questions_order();
        //Then
        let vec_questions = questions.return_questions(7);
        assert_eq!(vec_questions.len(), 7);
        println!("{:?}", vec_questions)
    }
}




