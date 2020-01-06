use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserScore{
    username: String,
    score :i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HighScores{
    pub scores: Vec<UserScore>
}

impl UserScore{
    pub fn new(username: String, score: i32) -> UserScore{
        UserScore{
            username: username,
            score: score
        }
    }
}

