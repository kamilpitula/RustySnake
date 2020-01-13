pub struct GameData {
    pub username: String
}

impl GameData{
    pub fn new() -> GameData{
        GameData{
            username: String::from("unknown")
        }
    }
}