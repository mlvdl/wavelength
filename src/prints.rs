use crate::colors::{BLUE, CYAN, GREEN, MAGENTA, RED, RESET, YELLOW};
use crate::{utils, GameState, WIDTH};
use crate::utils::clear_terminal;

pub fn print_lose() {
    print!("{} ", RED);
    println!("__   _____  _   _   _     ___  ____  _____  ");
    println!(" \\ \\ / / _ \\| | | | | |   / _ \\/ ___|| ____|");
    println!("  \\ V / | | | | | | | |  | | | \\___ \\|  _|");
    println!("   | || |_| | |_| | | |__| |_| |___) | |___");
    println!("   |_| \\___/ \\___/  |_____\\___/|____/|_____|");
    println!();
}

pub fn print_win() {
    print!("{} ", GREEN);
    println!();
    println!(" __   _____  _   _  __        _____ _   _  ");
    println!(" \\ \\ / / _ \\| | | | \\ \\      / /_ _| \\ | | ");
    println!("  \\ V / | | | | | |  \\ \\ /\\ / / | ||  \\| |");
    println!("   | || |_| | |_| |   \\ V  V /  | || |\\  |");
    println!("   |_| \\___/ \\___/     \\_/\\_/  |___|_| \\_|");
    println!();
}

pub fn print_banner() {
    print_hashtags();
    println!("{}                          _                  _   _             ", RED);
    println!("{} __      ____ ___   _____| | ___ _ __   __ _| |_| |__          " , YELLOW);
    println!("{} \\ \\ /\\ / / _` \\ \\ / / _ \\ |/ _ \\ '_ \\ / _` | __| '_ \\", GREEN);
    println!("{}  \\ V  V / (_| |\\ V /  __/ |  __/ | | | (_| | |_| | | |      ", CYAN);
    println!("{}   \\_/\\_/ \\__,_| \\_/ \\___|_|\\___|_| |_|\\__, |\\__|_| |_|", BLUE);
    println!("{}                                       |___/                   ", MAGENTA);
    println!("{}", RESET);
    print_hashtags();
}

pub fn print_help() {
    println!("Wavelength is played over rounds, where the teammates will alternate playing the psychic.");
    println!("Each round consists of 3 phases.");
    println!("1. Psychic Phase.");
    println!("2. Team Phase.");
    println!("3. Scoring Phase.");
    println!();
    println!("1. Psychic draws a card. Wavelength cards list 2 opposite ends of a spectrum.");
    println!("Psychic gets to see where the hidden target stands, and gives a clue for the target area's position.");
    println!("The clue can be up to 5 words, and should relate to the spectrum but be abstract enough to challenge the team.");
    println!("2. Teammates try to guess where the hidden target is in the spectrum.");
    println!("3. Scores: 4 point if the target position is guessed, 3 points for +/-1, 2 points for +/-2, 0 otherwise.");
    println!("\nGood luck!\n");
}

pub fn print_spectrum(start: i32, end: i32) {
    let mut message = String::new();
    for value in start..=end {
        let color = utils::get_color(value - start, end - start);
        message += &format!("{} {} {}", color, value, "\x1b[0m");
    }
    let message_length: usize = (start..end).map(|i| if i < 10 { 3 } else { 4 }).sum();
    let left_margin;
    if message_length < WIDTH {
        left_margin = (WIDTH - message_length) / 2 ;
    } else {
        left_margin = 0;
    }
    println!("{:left_margin$}{}", "", message, left_margin = left_margin);
    println!();
}

pub fn print_card(game_state: &GameState) {

    let max_length = game_state.card.len();
    let horizontal_border = format!("+{}+", "-".repeat(max_length + 2));

    println!("The drawn card is:");
    println!("{}", game_state.color_code);
    let left_margin = (WIDTH - horizontal_border.len()) / 2;
    println!("{:left_margin$}{}", "", horizontal_border, left_margin = left_margin);
    print!("{:left_margin$}|", "", left_margin = left_margin);
    println!(" \x1B[1m{:<width$}\x1B[0m{} |", game_state.card, game_state.color_code, width = max_length);
    println!("{:left_margin$}{}", "", horizontal_border, left_margin = left_margin);
    println!("{}", RESET);
}

pub fn print_final_scores(total_points: i32) {
    if total_points <= 3 { println!("{}Are you sure it’s plugged in?", RED); }
    else if total_points <= 6 { println!("{}Try turning it off and back on again.", YELLOW); }
    else if total_points <= 9 { println!("{}Blow into the bottom of the device.", GREEN); }
    else if total_points <= 12 { println!("{}Not bad! Not great, but not bad.", CYAN); }
    else if total_points <= 15 { println!("{}So close!", CYAN); } else if total_points <= 18 { println!("{}You won!", BLUE); }
    else if total_points <= 21 { println!("{}You’re on the same... wavelength.", BLUE); }
    else if total_points <= 24 { println!("{}Galaxy brain.", MAGENTA); }
    else { println!("{} Head exploding emoji!", MAGENTA) }

    if total_points > 15 { print_win(); } else { print_lose(); }
}

pub fn print_welcome_message() {
    clear_terminal();
    println!("Welcome to the digital cooperative version of Wavelength!\n");
    println!("How to play:");
    println!("The objective of Wavelength is to give your teammates a clue allowing them to \
    \naccurately predict where to target on a spectrum. \
    \nIf you get more than 15 points, you win!");
}

pub fn print_hashtags() {
    let start = 0;
    let end = WIDTH as i32;
    for value in start..=end {
        let color = utils::get_color(value - start, end - start);
        print!("{}#{}", color, "\x1b[0m");
    }
    println!();
}