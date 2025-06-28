use crate::config::game_config::GameConfig;
use crate::game::modes::GameMode;
use crate::GameState;

pub struct CompetitiveMode {
    pub config: GameConfig,
}

impl CompetitiveMode {
    pub fn new(config: GameConfig) -> Self {
        println!("The weak compete. The strong dominate.");
        CompetitiveMode { config }

    }
}

impl GameMode for CompetitiveMode {
    fn setup(&mut self) {
        println!("Setting up competitive mode");
    }
    
    fn play_round(&mut self) {
        println!("Playing round in competitive mode");
    }
    
    fn calculate_points(&self) -> u32 {
        self.config.points_to_win
    }
    
    fn check_win_condition(&self) -> bool {
        false
    }
}