pub struct CooperativeState {
    pub base: BaseGameState,
    pub score: u32,
    pub round: u32,
}

impl GameState for CooperativeState {
    fn update(&mut self) { /* update logic */ }
    fn is_game_over(&self) -> bool { self.team_lives == 0 }
    fn base(&self) -> &BaseGameState { &self.base }
    fn base_mut(&mut self) -> &mut BaseGameState { &mut self.base }
}
