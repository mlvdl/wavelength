#[derive(Debug)]
pub struct GameConfig {
    pub file: String,
    pub spectrum: (i32, i32),
    pub points_to_win: u32,
    pub number_of_teams: u8,
    pub n_rounds: Option<u8>,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            file: String::from("cards.txt"),
            spectrum: (0, 10),
            points_to_win: 15,
            number_of_teams: 1,
            n_rounds: Some(7),
        }
    }
}

impl GameConfig {
    pub fn new_cooperative() -> Self {
        GameConfig {
            ..Default::default()
        }
    }

    pub fn new_competitive() -> Self {
        GameConfig {
            points_to_win: 10,
            number_of_teams: 2,
            ..Default::default()
        }
    }
}
