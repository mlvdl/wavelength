use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::process::exit;
use rand::Rng;
use colors::{BLUE, COLORMAP, CYAN, GREEN, MAGENTA, RED, RESET, YELLOW};
use crate::utils::clear_terminal;

mod prints;
mod utils;
mod colors;


fn main() {
    println!("Welcome to the digital cooperative version of Wavelength!");
    prints::print_banner();
    println!("How to play:");
    println!("The objective of Wavelength is to give your teammates a clue allowing them to accurately predict where to target on a spectrum. \
    \nIf you get more than 15 points, you win!");

    // Define the spectrum
    let mut end = 10;
    start_menu(&mut end);
    let spectrum = 0..=end;
    clear_terminal();
    println!("The spectrum is: ");
    prints::print_spectrum(*spectrum.start(), *spectrum.end());

    // Draw card
    let path = Path::new("cards.txt");

    let file = File::open(path);
    let mut lines = Vec::new();
    for line in io::BufReader::new(file.unwrap()).lines() {
        lines.push(line.unwrap());
    }

    let mut total_points = 0;
    let n_rounds = 7;
    let mut answer = String::new();

    for round in 1..n_rounds + 1 {
        println!("\nPress enter to start round {}.", round);
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        clear_terminal();

        println!("{}###################################### ROUND {round} ########################################", COLORMAP[round - 1]);
        println!("{}", RESET);

        let mut card_content ;
        loop {
            answer.clear();
            let mut rng = rand::rng();
            let random_index = rng.random_range(0..lines.len());
            card_content = &lines[random_index];
            println!("You draw the following card:");
            prints::print_card(card_content);
            println!("Press enter to see the hidden target is. Psst... make sure that only the psychic sees it! (Press (n) to get a new card)");
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");
            let answer = answer.trim();
            if answer != "n" { break; }
        }

        // Randomly select a hidden target on the spectrum
        let mut rng = rand::rng();
        let hidden_target = rng.random_range(spectrum.clone());
        println!("The hidden target is at position: {}", hidden_target);

        // Wait for user input to clear the terminal
        println!("Press Enter to clear the terminal...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        utils::clear_terminal();

        println!("The card is:");
        prints::print_card(card_content);

        // The Psychic gives a clue
        println!("Psychic, please give a clue (e.g., a word or phrase):");
        let mut clue = String::new();
        io::stdin().read_line(&mut clue).expect("Failed to read line");

        // The Guesser tries to guess the position
        println!("Guesser, please guess the position on the spectrum ({}, {}):", *spectrum.start(), *spectrum.end());
        prints::print_spectrum(*spectrum.start(), *spectrum.end());

        let guess = utils::read_number(0, end);

        // Check if the guess is correct
        let round_points ;
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
        println!("The hidden target was at position {}.", hidden_target);

        total_points += round_points;
        print!("You got {} points in this round!", round_points);
        println!(" That's a total of {} points!", total_points);
    }

    if total_points <= 3 { println!("{}Are you sure it’s plugged in?", RED); }
    else if total_points <= 6 { println!("{}Try turning it off and back on again.", YELLOW); }
    else if total_points <= 9 { println!("{}Blow into the bottom of the device.", GREEN); }
    else if total_points <= 12 { println!("{}Not bad! Not great, but not bad.", CYAN); }
    else if total_points <= 15 { println!("{}So close!", CYAN); }
    else if total_points <= 18 { println!("{}You won!", BLUE); }
    else if total_points <= 21 { println!("{}You’re on the same... wavelength.", BLUE); }
    else if total_points <= 24 { println!("{}Galaxy brain.", MAGENTA); }
    else { println!("{} Head exploding emoji!", MAGENTA)}

    if total_points > 15 { prints::print_win(); }
    else { prints::print_lose();}
}

fn start_menu(end: &mut i32) {
    let mut answer = String::new();
    loop {
        println!("Press (p) to change the default parameters, (h) to see how to play, (q) to quit, enter (↵) to continue.");
        answer.clear();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        if answer.trim().to_string() == "h" {
            prints::print_help();
        } else if answer.trim().to_string() == "p" {
            println!("Set the upper limit of the spectrum:");
            *end = utils::read_number(0, 100);
        } else if answer.trim().to_string() == "q" {
            exit(0);
        } else {
            break
        }
    }
}
