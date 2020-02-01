use std::fs::File;
use std::io::prelude::*;
use super::userscore::{UserScore, HighScores};

pub struct ScoreController {
    high_scores: HighScores
}

impl ScoreController{
    pub fn new() -> ScoreController {
        let file = File::open("high.scores");
        let deserialized_scores: HighScores;
        
        match file {
            Ok(mut f) => {
                let mut contents = String::new(); 
                f.read_to_string(&mut contents).expect("Read file failed");
                deserialized_scores = serde_yaml::from_str(&contents).unwrap();
            },
            Err(_e) => deserialized_scores = HighScores{ scores: Vec::new()},
        }

        ScoreController {
            high_scores: deserialized_scores
        }
    }

    pub fn add_new_score(&mut self, username: &str, score: i32){
        let new_score = UserScore::new(username.to_string(), score);
        self.high_scores.scores.push(new_score);
        
        let serialized = serde_yaml::to_string(&self.high_scores).unwrap();

        let mut file = File::create("high.scores").unwrap();
        file.write_all(&serialized.as_bytes()).expect("File write failed");
    }

    pub fn get_high_score(&mut self) -> String {
        if self.high_scores.scores.len() > 0 {
            self.high_scores.scores.sort_by(|a, b| b.score.cmp(&a.score));
            self.high_scores.scores
                .iter()
                .take(1)
                .map(|x| x.score.to_string())
                .collect()
        } else {
            "0".to_string()
        }
    }

    pub fn get_top_scores(&mut self, count: usize) -> Vec<String> {
        if self.high_scores.scores.len() > 0 {
            self.high_scores.scores.sort_by(|a, b| b.score.cmp(&a.score));
            return self.high_scores.scores
                        .iter()
                        .take(count)
                        .map(|x| x.username.to_string() + "  " + &x.score.to_string())
                        .collect();
        } else {
            return Vec::new();
        }
    }
}