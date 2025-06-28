use std::fs::File;
use std::io;
use std::io::BufRead;

use crate::colors::RESET;
use rand::Rng;
use std::process::exit;

use crate::GameConfig;
use crate::GameState;

use crate::prints;
use crate::utils;

pub struct CooperativeMode {
    state: GameState,
    config: GameConfig,
}

impl CooperativeMode {
    pub fn new(config: GameConfig) -> Self {
        let state = GameState::new();
        CooperativeMode { state, config }
    }

    fn clear(game_config: &GameConfig, game_state: &GameState) {
        utils::clear_terminal();
        println!(
            "{}############################### ROUND {} / {} ################################",
            game_state.color_code, game_state.round, game_config.n_rounds
        );
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
                CooperativeMode::clear(&game_config, &game_state);
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
                CooperativeMode::clear(&game_config, &game_state)
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
                game_config.spectrum.1 = utils::read_number(0, 100);
            } else if answer.trim().to_string() == "q" {
                exit(0);
            } else {
                break;
            }
        }
    }
}

impl CooperativeMode {
    pub fn play(&mut self) {
        Self::start_menu(&mut self.config);

        for round in 0..self.config.n_rounds {
            self.state.round = round + 1;
            self.state.color_code =
                utils::get_color(self.state.round as i32, self.config.n_rounds as i32);

            let msg = format!("\nPress enter (↵) to start round {}.", self.state.round);
            CooperativeMode::wait_for_enter(&self.state, msg);

            self.play_round();
        }

        prints::print_final_scores(self.state.score as i32);
    }

    fn play_round(&mut self) {
        CooperativeMode::clear(&self.config, &self.state);
        let lines = CooperativeMode::read_cards(&mut self.config);
        CooperativeMode::draw_card(&lines, &self.config, &mut self.state);
        CooperativeMode::get_hidden_target(&mut self.config, &mut self.state);

        CooperativeMode::clear(&self.config, &self.state);

        prints::print_card(&self.state);

        CooperativeMode::wait_for_enter(
            &self.state,
            "Psychic, please give a clue (e.g., a word or phrase)".to_string(),
        );
        CooperativeMode::clear(&self.config, &self.state);

        prints::print_card(&self.state);

        println!(
            "Guesser, please guess the position on the spectrum ({}, {}):",
            self.config.spectrum.0, self.config.spectrum.1
        );
        let guess = utils::read_number(self.config.spectrum.0, self.config.spectrum.1);

        let round_points = CooperativeMode::get_round_points(self.state.target, guess);
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

    // pub fn end_game(&self) {
    //     println!("Cooperative game ended. Final state: {:?}", self.state);
    // }
}
