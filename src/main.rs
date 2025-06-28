mod colors;
mod cooperative;
mod prints;
mod utils;

const WIDTH: usize = 75;

#[derive(Debug)]
struct GameConfig {
    spectrum: (i32, i32),
    n_rounds: usize,
    file: String,
}
struct GameState {
    round: usize,
    score: i32,
    color_code: String,
    card: String,
    target: i32,
}

impl GameState {
    fn new() -> Self {
        GameState {
            round: 0,
            score: 0,
            color_code: String::new(),
            card: String::new(),
            target: -1,
        }
    }
}

fn main() {
    prints::print_welcome_message();

    let game_config = GameConfig {
        spectrum: (0, 10),
        n_rounds: 7,
        file: String::from("cards.txt"),
    };

    println!("The spectrum is: ");
    prints::print_spectrum(game_config.spectrum.0, game_config.spectrum.1);

    let mut game = cooperative::CooperativeMode::new(game_config);
    game.play();
}
