pub mod competitive;
pub mod cooperative;

pub(crate) trait GameMode {
    fn play(&mut self);
}
