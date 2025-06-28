mod game;
mod config;
mod round;
mod utils;
mod settings;

use game::modes::{cooperative::CooperativeMode, competitive::CompetitiveMode, ActiveGameMode};
use game::modes::GameMode;
use config::game_config::GameConfig;
use crate::settings::N_TEAMS;
use crate::utils::general::read_number;
use crate::utils::prints;

struct GameParams {
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

fn main() {
    prints::print_welcome_message();
    println!("Choose the number of teams 1 or 2. [default: {N_TEAMS}]");
    let n_teams = read_number(1, 2, Some(N_TEAMS as i32));
    let mut game = match n_teams {
        1 => ActiveGameMode::Cooperative(CooperativeMode::new(GameConfig::new_cooperative())),
        2 => ActiveGameMode::Competitive(CompetitiveMode::new(GameConfig::new_competitive())),
        _ => panic!("Invalid choice for number of teams"),
    };

    // // Example: Starting the game in cooperative mode
    // let cooperative_config = GameConfig::new_cooperative();
    // let cooperative_mode = CooperativeMode::new(cooperative_config);
    // let mut game = ActiveGameMode::Cooperative(cooperative_mode);
    //
    match game {
        ActiveGameMode::Cooperative(ref mut mode) => {
            mode.setup();
            for round in 1..mode.config.n_rounds.unwrap() + 1 {
                println!("Round {}", round);
                mode.play_round();
            }
        },
        ActiveGameMode::Competitive(ref mut mode) => {
            mode.setup();
            while !mode.check_win_condition() {
                mode.play_round();
                break;

            }
        },
        _ => unreachable!(),
    }
    //
    // // Example: Starting the game in competitive mode
    // let competitive_config = GameConfig::new_competitive();
    // let competitive_mode = CompetitiveMode::new(competitive_config);
    // let mut game = ActiveGameMode::Competitive(competitive_mode);
    //
    // match game {
    //     ActiveGameMode::Competitive(ref mut mode) => {
    //         mode.setup();
    //         while !mode.check_win_condition() {
    //             mode.play_round();
    //         }
    //     },
    //     _ => unreachable!(),
    // }
}
