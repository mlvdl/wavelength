pub struct GameConfig {
    pub points_to_win: u32,
    pub number_of_teams: u32,
    // Other configuration parameters
}

impl GameConfig {
    pub fn new_cooperative() -> Self {
        GameConfig {
            points_to_win: 100,
            number_of_teams: 1,
            // Other cooperative-specific settings
        }
    }

    pub fn new_competitive() -> Self {
        GameConfig {
            points_to_win: 200,
            number_of_teams: 2,
            // Other competitive-specific settings
        }
    }
}
