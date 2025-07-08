use crate::game_mode::GameMode;
use std::io;

use crate::colors::RESET;
use crate::game_config::GameConfig;
use crate::game_state::GameState;
use crate::{prints, utils};
use rand::Rng;

pub struct CooperativeMode {
    state: GameState,
    pub config: GameConfig,
}

impl CooperativeMode {
    pub fn new(config: GameConfig) -> Self {
        let state = GameState::new();
        CooperativeMode { state, config }
    }

    fn clear(&mut self) {
        utils::clear_terminal();
        if self.config.n_rounds.is_some() {
            println!(
                "{}##################### ROUND {} / {} ######################",
                self.state.color_code,
                self.state.round,
                self.config.n_rounds.unwrap()
            );
        }
        println!("Score: {}", self.state.score);
        println!("{}", RESET);
        println!("Spectrum:");
        prints::print_spectrum(self.config.spectrum.0, self.config.spectrum.1);
    }

    fn wait_for_enter(&mut self, message: String) {
        println!("{}", message);
        let mut answer = String::new();
        if self.state.target != -1 {
            println!("If you need to see the hidden target again press (t).");
        }
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        if answer.trim().to_lowercase() == "t" {
            println!("Hidden target position: {}", self.state.target);
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");
        }
    }

    fn get_hidden_target(&mut self) {
        let mut answer = String::new();
        loop {
            answer.clear();
            // Randomly select a hidden target on the spectrum
            self.state.target =
                rand::rng().random_range(self.config.spectrum.0..=self.config.spectrum.1);
            println!("The hidden target is at position: {}", self.state.target);

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
                self.clear();
                prints::print_card(&self.state);
            }
        }
    }
    fn get_round_points(hidden_target: i32, guess: i32) -> i32 {
        let round_points;
        if guess == hidden_target {
            round_points = 4;
            println!("Congratulations!")
        } else if guess - 1 == hidden_target || guess + 1 == hidden_target {
            round_points = 3;
            println!("Quite close!");
        } else if guess - 2 == hidden_target || guess + 2 == hidden_target {
            round_points = 2;
            println!("Not bad!");
        } else {
            round_points = 0;
            println!("Sorry, that wasn't even close...");
        }
        round_points
    }

    fn draw_card(&mut self) {
        let mut answer = String::new();
        let lines = utils::read_lines(&self.config.file);
        loop {
            answer.clear();
            let random_index = rand::rng().random_range(0..lines.len());
            self.state.card = lines[random_index].clone();
            // println!("You draw the following card:");
            prints::print_card(&self.state);
            println!(
                "Press enter (↵) to see the hidden target is. Psst... make sure that only the Psychic sees it!"
            );
            println!("(Press (n) if you need a new card)");
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");
            let answer = answer.trim();
            if answer != "n" {
                break;
            } else {
                self.clear()
            }
        }
    }

    fn play_round(&mut self) {
        self.clear();
        self.draw_card();
        self.get_hidden_target();
        self.clear();
        prints::print_card(&self.state);
        self.wait_for_enter("Psychic, please give a clue (e.g., a word or phrase)".to_string());
        self.clear();
        prints::print_card(&self.state);
        println!(
            "Guesser, please guess the position on the spectrum ({}, {}):",
            self.config.spectrum.0, self.config.spectrum.1
        );
        let guess = utils::read_number(self.config.spectrum.0, self.config.spectrum.1, None);
        let round_points = Self::get_round_points(self.state.target, guess);
        let color_code = utils::get_color(self.state.target, self.config.spectrum.1);
        println!(
            "The hidden target was at position {}{}{}.",
            color_code, self.state.target, RESET
        );
        self.state.target = -1;
        self.state.score += round_points;
        print!("You got {} points in this round!", round_points);
        println!(" That's a total of {} points!", self.state.score);
    }
}

impl GameMode for CooperativeMode {
    fn play(&mut self) {
        for round in 0..self.config.n_rounds.unwrap() {
            self.state.round = round + 1;
            self.state.color_code = utils::get_color(
                self.state.round as i32,
                self.config.n_rounds.unwrap() as i32,
            );

            let msg = format!("\nPress enter (↵) to start round {}.", self.state.round);
            self.wait_for_enter(msg);
            self.play_round();
        }

        prints::print_final_scores(self.state.score as i32);
    }
}
