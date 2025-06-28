pub mod cooperative;
pub mod competitive;

pub enum ActiveGameMode {
    Cooperative(cooperative::CooperativeMode),
    Competitive(competitive::CompetitiveMode),
}

pub(crate) trait GameMode {
    fn setup(&mut self);
    fn play_round(&mut self);
    fn calculate_points(&self) -> u32;
    fn check_win_condition(&self) -> bool;
}
