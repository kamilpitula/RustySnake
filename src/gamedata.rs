pub struct GameData {
    pub username: String,
    pub score: i32
}

impl GameData{
    pub fn new() -> GameData{
        GameData{
            username: String::from("unknown"),
            score: 0
        }
    }
}