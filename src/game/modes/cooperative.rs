use crate::config::game_config::GameConfig;
use crate::game::modes::GameMode;

pub struct CooperativeMode {
    pub config: GameConfig,

}

impl CooperativeMode {
    pub fn new(config: GameConfig) -> Self {
        println!("Unity makes strength.");
        CooperativeMode { config }    
    }
}

impl GameMode for CooperativeMode {
    fn setup(&mut self) {
        println!("Setting up cooperative mode");
        dbg!("{:?}", &self.config);
    }
    
    fn play_round(&mut self) {
        println!("Playing round in cooperative mode");
    }
    
    fn calculate_points(&self) -> u32 {
        self.config.points_to_win
    }
    
    fn check_win_condition(&self) -> bool {
        false
    }
}