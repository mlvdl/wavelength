pub struct GameState {
    pub name: String,
    pub round: u8,
    pub score: i32,
    pub color_code: String,
    pub card: String,
    pub target: i32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            name: "".to_string(),
            round: 0,
            score: 0,
            color_code: String::new(),
            card: String::new(),
            target: -1,
        }
    }
}
