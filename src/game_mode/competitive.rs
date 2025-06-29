use crate::game_mode::GameMode;
use std::fs::File;
use std::io;
use std::io::BufRead;

use crate::colors::{BLUE, RED, RESET};
use crate::game_config::GameConfig;
use crate::game_state::GameState;
use crate::{prints, utils};
use rand::Rng;
use std::process::exit;
pub struct CompetitiveMode {
    pub config: GameConfig,
    team1: GameState,
    team2: GameState,
}

impl CompetitiveMode {
    pub fn new(config: GameConfig) -> Self {
        CompetitiveMode {
            config,
            team1: GameState {
                round: 0,
                score: 0,
                color_code: RED.to_string(),
                card: String::new(),
                target: -1,
            },
            team2: GameState {
                round: 0,
                score: 1,
                color_code: BLUE.to_string(),
                card: String::new(),
                target: -1,
            },
        }
    }

    fn clear(game_config: &GameConfig, game_state: &GameState) {
        utils::clear_terminal();
        if game_config.n_rounds.is_some() {
            println!(
                "{}############################### ROUND {} / {} ################################",
                game_state.color_code,
                game_state.round,
                game_config.n_rounds.unwrap()
            );
        }
        println!("Score: {}", game_state.score);
        println!("{}", RESET);
        println!("Spectrum:");
        prints::print_spectrum(game_config.spectrum.0, game_config.spectrum.1);
    }

    fn wait_for_enter(game_state: &GameState, message: String) {
        println!("{}", message);
        let mut answer = String::new();
        if game_state.target != -1 {
            println!("If you need to see the hidden target again press (t).");
        }
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        if answer.trim().to_lowercase() == "t" {
            println!("Hidden target position: {}", game_state.target);
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");
        }
    }

    fn get_round_points(hidden_target: i32, guess: i32) -> i32 {
        let round_points;
        if guess == hidden_target {
            round_points = 4;
            print!("Congratulations!")
        } else if guess - 1 == hidden_target || guess + 1 == hidden_target {
            round_points = 3;
            print!("Quite close!");
        } else if guess - 2 == hidden_target || guess + 2 == hidden_target {
            round_points = 2;
            print!("Not bad!");
        } else {
            round_points = 0;
            print!("Sorry, that wasn't even close...");
        }
        round_points
    }

    fn get_hidden_target(game_config: &GameConfig, game_state: &mut GameState) {
        let mut answer = String::new();
        loop {
            answer.clear();
            // Randomly select a hidden target on the spectrum
            game_state.target =
                rand::rng().random_range(game_config.spectrum.0..=game_config.spectrum.1);
            println!("The hidden target is at position: {}", game_state.target);

            // Wait for user input to clear the terminal
            println!(
                "Press enter (↵) to clear the terminal. (Press (n) if you need a new hidden target)"
            );
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");

            let answer = answer.trim();
            if answer != "n" {
                break;
            } else {
                Self::clear(&game_config, &game_state);
                prints::print_card(&game_state);
            }
        }
    }

    fn read_cards(game_config: &GameConfig) -> Vec<String> {
        let file = File::open(game_config.file.clone());
        let mut lines = Vec::new();
        for line in io::BufReader::new(file.unwrap()).lines() {
            lines.push(line.unwrap());
        }
        lines
    }

    fn draw_card(lines: &Vec<String>, game_config: &GameConfig, game_state: &mut GameState) {
        let mut answer = String::new();
        loop {
            answer.clear();
            let random_index = rand::rng().random_range(0..lines.len());
            game_state.card = lines[random_index].clone();
            println!("You draw the following card:");
            prints::print_card(&game_state);
            println!(
                "Press enter (↵) to see the hidden target is. Psst... make sure that only the psychic sees it!"
            );
            println!("(Press (n) if you need a new card)");
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");
            let answer = answer.trim();
            if answer != "n" {
                break;
            } else {
                Self::clear(&game_config, &game_state)
            }
        }
    }
    fn start_menu(game_config: &mut GameConfig) {
        let mut answer = String::new();
        loop {
            println!(
                "Press (s) to change the spectrum, (h) to see how to play, (q) to quit, enter (↵) to continue."
            );
            answer.clear();
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");
            if answer.trim().to_string() == "h" {
                prints::print_help();
            } else if answer.trim().to_string() == "s" {
                println!("Set the upper limit of the spectrum:");
                game_config.spectrum.1 = utils::read_number(0, 100, None);
            } else if answer.trim().to_string() == "q" {
                exit(0);
            } else {
                break;
            }
        }
    }
    fn play_round(config: &GameConfig, mut game_state: &mut GameState) {
        Self::clear(&config, &game_state);
        let lines = Self::read_cards(&config);
        Self::draw_card(&lines, &config, &mut game_state);
        Self::get_hidden_target(&config, &mut game_state);
        Self::clear(&config, &game_state);
        prints::print_card(&game_state);
        Self::wait_for_enter(
            &game_state,
            "Psychic, please give a clue (e.g., a word or phrase)".to_string(),
        );
        Self::clear(&config, &game_state);
        prints::print_card(&game_state);
        println!(
            "Guesser, please guess the position on the spectrum ({}, {}):",
            config.spectrum.0, config.spectrum.1
        );
        let guess = utils::read_number(config.spectrum.0, config.spectrum.1, None);
        let round_points = Self::get_round_points(game_state.target, guess);
        let color_code = utils::get_color(game_state.target, config.spectrum.1);
        println!(
            "The hidden target was at position {}{}{}.",
            color_code, game_state.target, RESET
        );
        game_state.target = -1;
        game_state.score += round_points;
        print!("You got {} points in this round!", round_points);
        println!(" That's a total of {} points!", game_state.score);
    }
}

impl GameMode for CompetitiveMode {
    fn play(&mut self) {
        Self::start_menu(&mut self.config);

        loop {
            Self::wait_for_enter(
                &self.team1,
                "\nPress enter (↵) to start round for team 1".to_string(),
            );
            Self::play_round(&self.config, &mut self.team1);
            if self.team1.score > self.config.points_to_win {
                break;
            };
            Self::wait_for_enter(
                &self.team1,
                "\nPress enter (↵) to start round for team 1".to_string(),
            );
            if self.team1.score > self.config.points_to_win {
                break;
            };
            Self::play_round(&self.config, &mut self.team2);
        }
    }
}
