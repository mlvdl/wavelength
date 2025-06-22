use std::fs::File;
use std::io;
use std::io::BufRead;

use std::process::exit;
use rand::Rng;
use crate::colors::RESET;

mod prints;
mod utils;
mod colors;


const WIDTH: usize = 75;

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

    // Define the spectrum
    let mut game_params = GameParams {
        spectrum: (0, 10),
        n_rounds: 7,
        file: String::from("cards.txt"),
        
    };
    let mut game_state = GameState {
        round: 0,
        score: 0,
        color_code: RESET.to_string(),
        card: String::new(),
        target: -1,
    };
    start_menu(&mut game_params);
    utils::clear_terminal();

    println!("The spectrum is: ");
    prints::print_spectrum(game_params.spectrum.0, game_params.spectrum.1);

    let lines = read_cards(&mut game_params);

    let color_map = utils::get_color_map(0, game_params.n_rounds as i32);
    for round in 0..game_params.n_rounds {
        game_state.round = round + 1;
        game_state.color_code = color_map[game_state.round].clone();

        wait_for_enter(&game_state, format!("\nPress enter (↵) to start round {}.", game_state.round));
        clear(&game_params, &game_state);

        draw_card(&lines, &game_params, &mut game_state);
        get_hidden_target(&mut game_params, &mut game_state);

        clear(&game_params, &game_state);

        prints::print_card(&game_state);

        wait_for_enter(&game_state, "Psychic, please give a clue (e.g., a word or phrase)".to_string());
        clear(&game_params, &game_state);

        prints::print_card(&game_state);

        println!("Guesser, please guess the position on the spectrum ({}, {}):", game_params.spectrum.0, game_params.spectrum.1);
        let guess = utils::read_number(game_params.spectrum.0, game_params.spectrum.1);

        let round_points = get_round_points(game_state.target, guess);
        let color_code = utils::get_color(game_state.target, game_params.spectrum.1);
        println!("The hidden target was at position {}{}{}.", color_code, game_state.target, RESET);
        game_state.target = -1;

        game_state.score += round_points;
        print!("You got {} points in this round!", round_points);
        println!(" That's a total of {} points!", game_state.score);
    }
    prints::print_final_scores(game_state.score);
}

fn clear(game_params: &GameParams, game_state: &GameState) {
    utils::clear_terminal();
    println!("{}############################### ROUND {} / {} ################################",
             game_state.color_code, game_state.round, game_params.n_rounds);
    println!("Score: {}", game_state.score);
    println!("{}", RESET);
    println!("Spectrum:");
    prints::print_spectrum(game_params.spectrum.0, game_params.spectrum.1);

}

fn wait_for_enter(game_state: &GameState, message: String) {
    println!("{}", message);
    let mut answer = String::new();
    if game_state.target != -1 {
        println!("If you need to see the hidden target again press (t).");
    }
    io::stdin().read_line(&mut answer).expect("Failed to read line");
    if answer.trim().to_lowercase() == "t" {
        println!("Hidden target position: {}", game_state.target);
        io::stdin().read_line(&mut answer).expect("Failed to read line");
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

fn get_hidden_target(game_params: &GameParams, game_state: &mut GameState) {
    let mut answer = String::new();
    loop {
        answer.clear();
        // Randomly select a hidden target on the spectrum
        game_state.target = rand::rng().random_range(game_params.spectrum.0..=game_params.spectrum.1);
        println!("The hidden target is at position: {}", game_state.target);

        // Wait for user input to clear the terminal
        println!("Press enter (↵) to clear the terminal. (Press (n) if you need a new hidden target)");
        io::stdin().read_line(&mut answer).expect("Failed to read line");

        let answer = answer.trim();
        if answer != "n" { break; } else {
            clear(&game_params, &game_state);
            prints::print_card(&game_state);
        }
    }
}

fn read_cards(game_params: &GameParams) -> Vec<String> {
    let file = File::open(game_params.file.clone());
    let mut lines = Vec::new();
    for line in io::BufReader::new(file.unwrap()).lines() {
        lines.push(line.unwrap());
    }
    lines
}

fn draw_card(lines: &Vec<String>, game_params: &GameParams, game_state: &mut GameState) {
    let mut answer = String::new();

    loop {
        answer.clear();
        let random_index = rand::rng().random_range(0..lines.len());
        game_state.card = lines[random_index].clone();
        println!("You draw the following card:");
        prints::print_card(&game_state);
        println!("Press enter (↵) to see the hidden target is. Psst... make sure that only the psychic sees it!");
        println!("(Press (n) if you need a new card)");
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        let answer = answer.trim();
        if answer != "n" { break; } else { clear(&game_params, &game_state) }
    }
}

fn start_menu(game_params: &mut GameParams) {
    let mut answer = String::new();
    loop {
        println!("Press (p) to change the default parameters, (h) to see how to play, (q) to quit, enter (↵) to continue.");
        answer.clear();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        if answer.trim().to_string() == "h" {
            prints::print_help();
        } else if answer.trim().to_string() == "p" {
            println!("Set the upper limit of the spectrum:");
            game_params.spectrum.1 = utils::read_number(0, 100);
        } else if answer.trim().to_string() == "q" {
            exit(0);
        } else {
            break
        }
    }
}
