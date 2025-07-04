use crate::settings::{N_TEAMS, SPECTRUM};
use crate::{prints, utils};
use std::io;
use std::process::exit;

#[derive(Debug)]
pub struct GameConfig {
    pub file: String,
    pub spectrum: (i32, i32),
    pub points_to_win: i32,
    pub number_of_teams: u8,
    pub n_rounds: Option<u8>,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            file: String::from("cards.txt"),
            spectrum: SPECTRUM,
            points_to_win: 15,
            number_of_teams: N_TEAMS,
            n_rounds: Some(7),
        }
    }
}

impl GameConfig {
    pub fn new_cooperative(spectrum_top: i32) -> Self {
        GameConfig {
            spectrum: (SPECTRUM.0, spectrum_top),
            ..Default::default()
        }
    }

    pub fn new_competitive(spectrum_top: i32) -> Self {
        GameConfig {
            points_to_win: 10,
            number_of_teams: 2,
            spectrum: (SPECTRUM.0, spectrum_top),
            ..Default::default()
        }
    }
}

pub fn start_menu() -> i32 {
    let mut answer = String::new();
    let mut spectrum_top: i32 = SPECTRUM.1;
    loop {
        println!(
            "Press enter (↵) to continue, (h) to see how to play, (d) to choose the difficulty, (q) to quit."
        );
        answer.clear();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        if answer.trim().to_string() == "h" {
            prints::print_help();
        } else if answer.trim().to_string() == "d" {
            println!("Set the level of difficulty:\
            \n 1. Easy\
            \n 2. Medium\
            \n 3. Difficult\
            ");
            let difficulty = utils::read_number(1, 3, Some(1));
            match difficulty {
                1 => spectrum_top = 10,
                2 => spectrum_top = 15,
                3 => spectrum_top = 20,
                _ => println!("Invalid difficulty"),
            }
        } else if answer.trim().to_string() == "q" {
            exit(0);
        } else {
            return spectrum_top;
        }
    }
}
