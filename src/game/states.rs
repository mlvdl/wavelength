pub mod cooperative;
pub mod competitive;


#[derive(Debug)]
pub struct BaseGameState {
    pub level: u32,
    pub color_code: String,
    pub card: String,
    pub target: i32,
    // other common properties
}

pub trait GameState {
    fn update(&mut self);
    fn is_game_over(&self) -> bool;
    fn base(&self) -> &BaseGameState;
    fn base_mut(&mut self) -> &mut BaseGameState;
}
