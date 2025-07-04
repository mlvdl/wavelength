use crate::game_config::GameConfig;
use crate::game_mode::GameMode;
use crate::game_mode::competitive::CompetitiveMode;
use crate::game_mode::cooperative::CooperativeMode;
use settings::N_TEAMS;

mod colors;
mod game_config;
mod game_mode;
mod game_state;
mod prints;
mod settings;
mod utils;

fn main() {
    prints::print_welcome_message();
    // println!("\nThe spectrum is: ");
    // prints::print_spectrum(SPECTRUM.0, SPECTRUM.1);
    let spectrum_top = game_config::start_menu();

    println!("Choose the number of teams 1 or 2. [default: {N_TEAMS}]");
    let n_teams = utils::read_number(1, 2, Some(N_TEAMS as i32));

    let mut game: Box<dyn GameMode> = match n_teams {
        1 => Box::new(CooperativeMode::new(GameConfig::new_cooperative(
            spectrum_top,
        ))),
        2 => Box::new(CompetitiveMode::new(GameConfig::new_competitive(
            spectrum_top,
        ))),
        _ => unreachable!(),
    };

    game.play();
}
