
pub struct CompetitiveState {
    pub base: BaseGameState,
    pub team_scores: Vec<u32>, // competitive-specific
}

impl GameState for CompetitiveState {
    fn update(&mut self) { /* update logic */ }
    fn is_game_over(&self) -> bool { self.player_scores.iter().any(|&s| s >= 100) }
    fn base(&self) -> &BaseGameState { &self.base }
    fn base_mut(&mut self) -> &mut BaseGameState { &mut self.base }
}
