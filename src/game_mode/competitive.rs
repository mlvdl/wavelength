use crate::colors::RESET;
use crate::game_config::GameConfig;
use crate::game_mode::GameMode;
use crate::game_state::GameState;
use crate::prints::{print_spectrum, print_team1_wins, print_team2_wins};
use crate::settings::WIDTH;
use crate::utils::get_color;
use crate::{prints, utils};
use rand::Rng;
use std::io;

pub struct CompetitiveMode {
    pub config: GameConfig,
    team1: GameState,
    team2: GameState,
}

impl CompetitiveMode {
    pub fn new(config: GameConfig) -> Self {
        let rng1 = rand::random_range(0..=10);
        let mut rng2 = rand::random_range(0..=10);
        while rng2 == rng1 {
            rng2 = rand::random_range(0..=10);
        }
        CompetitiveMode {
            config,
            team1: GameState {
                name: "Team1".to_string(),
                round: 0,
                score: 0,
                color_code: get_color(rng1, 10).to_string(),
                card: String::new(),
                target: -1,
            },
            team2: GameState {
                name: "Team2".to_string(),
                round: 0,
                score: 1,
                color_code: get_color(rng2, 10).to_string(),
                card: String::new(),
                target: -1,
            },
        }
    }

    fn clear(&mut self) {
        utils::clear_terminal();
        let mut w = WIDTH / 2;
        for _ in 0..w + 1 {
            print!("{}{}", self.team1.color_code, "#");
        }
        for _ in 0..w + 1 {
            print!("{}{}", self.team2.color_code, "#");
        }
        println!();
        print!("{} Score: {}", self.team1.color_code, self.team1.score);
        w = WIDTH - 18;
        for _ in 0..w {
            print!(" ");
        }
        print!("{} Score: {}", self.team2.color_code, self.team2.score);
        println!("{}\n", RESET);
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
                // Self::clear();
                prints::print_card(&game_state);
            }
        }
    }
    fn get_round_points(hidden_target: i32, guess: i32) -> i32 {
        let round_points;
        if guess == hidden_target {
            round_points = 4;
            // println!("Congratulations!")
        } else if guess - 1 == hidden_target || guess + 1 == hidden_target {
            round_points = 3;
            // println!("Quite close!");
        } else if guess - 2 == hidden_target || guess + 2 == hidden_target {
            round_points = 2;
            // println!("Not bad!");
        } else {
            round_points = 0;
            // println!("Sorry, that wasn't even close...");
        }
        round_points
    }

    fn draw_card(game_config: &GameConfig, game_state: &mut GameState) {
        let mut answer = String::new();
        loop {
            answer.clear();
            let lines = utils::read_lines(&game_config.file);
            let random_index = rand::rng().random_range(0..lines.len());
            game_state.card = lines[random_index].clone();
            // println!(
            //     "{}{}{} draw the following card:",
            //     game_state.color_code, game_state.name, RESET
            // );
            prints::print_card(&game_state);
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
                // Self::clear(&game_config, &game_state)
            }
        }
    }

    fn play_round(config: &GameConfig, mut game_state: &mut GameState) -> i32 {
        Self::draw_card(&config, &mut game_state);
        Self::get_hidden_target(&config, &mut game_state);

        utils::clear_terminal();
        prints::print_card(&game_state);
        Self::wait_for_enter(
            &game_state,
            "Psychic, please give a clue (e.g., a word or phrase)".to_string(),
        );
        println!(
            "Guesser, please guess the position on the spectrum ({}, {}):",
            config.spectrum.0, config.spectrum.1
        );
        print_spectrum(config.spectrum.0, config.spectrum.1);
        let guess = utils::read_number(config.spectrum.0, config.spectrum.1, None);

        println!("Now the other team to guess \x1B[1mleft (l)\x1B[0m or \x1B[1mright (r)\x1B[0m.");
        let mut left_right_guess = String::new();
        io::stdin()
            .read_line(&mut left_right_guess)
            .expect("Failed to read line");
        while !left_right_guess.trim().to_lowercase().starts_with('l')
            && !left_right_guess.trim().to_lowercase().starts_with('r')
        {
            println!("Your guess {}", left_right_guess.trim().to_lowercase());
            left_right_guess.clear();
            println!(
                "Not a valid choice. Enter \x1B[1mleft (l)\x1B[0m or \x1B[1mright (r)\x1B[0m."
            );
            io::stdin()
                .read_line(&mut left_right_guess)
                .expect("Failed to read line");
        }
        let mut other_teams_point = 0;

        if left_right_guess.trim().to_lowercase().starts_with('l') && guess > game_state.target {
            other_teams_point += 1;
        } else if left_right_guess.trim().to_lowercase().starts_with('r')
            && guess < game_state.target
        {
            other_teams_point += 1;
        }

        let round_points = Self::get_round_points(game_state.target, guess);
        let color_code = get_color(game_state.target, config.spectrum.1);
        println!(
            "\nThe hidden target was at position {}{}{}.",
            color_code, game_state.target, RESET
        );
        game_state.target = -1;
        game_state.score += round_points;
        print!("\n{}{}{} ", game_state.color_code, game_state.name, RESET);
        print!(
            "got {}{}{} points in this round!",
            game_state.color_code, round_points, RESET
        );
        println!(
            " That's a total of {}{}{} point(s)!",
            game_state.color_code, game_state.score, RESET
        );
        other_teams_point
    }

    fn check_win(&mut self) -> bool {
        if self.team1.score >= self.config.points_to_win {
            print_team1_wins(&self.team1.color_code);
            return true;
        };
        if self.team2.score >= self.config.points_to_win {
            print_team2_wins(&self.team2.color_code);
            return true;
        };
        false
    }
}

impl GameMode for CompetitiveMode {
    fn play(&mut self) {
        loop {
            Self::wait_for_enter(
                &self.team1,
                format!("\n\nPress enter (↵) to start round for {}", self.team1.name),
            );
            self.clear();
            let team2_round_score = Self::play_round(&self.config, &mut self.team1);
            if team2_round_score > 0 {
                print!(
                    "\n{}{}{}: got an extra point!",
                    self.team2.color_code, self.team2.name, RESET
                );
                self.team2.score += team2_round_score;
            } else {
                print!(
                    "\n{}{}{}: No extra point this time.",
                    self.team2.color_code, self.team2.name, RESET
                );
            }
            if self.check_win() {
                break;
            }
            Self::wait_for_enter(
                &self.team2,
                format!("\n\nPress enter (↵) to start round for {}", self.team2.name),
            );
            self.clear();
            let team1_round_score = Self::play_round(&self.config, &mut self.team2);
            if team1_round_score > 0 {
                print!(
                    "\n{}{}{} got an extra point!",
                    self.team1.color_code, self.team1.name, RESET
                );
                self.team1.score += team1_round_score;
            } else {
                print!(
                    "\n{}{}{}: No extra point this time.",
                    self.team1.color_code, self.team1.name, RESET
                );
            }
            if self.check_win() {
                break;
            }
        }
    }
}
