use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use rand::Rng;
use colors::{BLUE, COLORMAP, CYAN, GREEN, MAGENTA, RED, RESET, YELLOW};

mod prints;
mod utils;
mod colors;


fn main() {
    println!("Welcome to the digital cooperative version of Wavelength!");
    prints::print_banner();
    println!("How to play:");
    println!("The objective of Wavelength is to give your teammates a clue allowing them to accurately predict where to target on a spectrum. \
    \nIf you get more than 15 points, you win!");

    println!("Press (h) if you need more help, enter to continue.");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read line");
    if answer.starts_with('h') {
        prints::print_help();
    }

    // Define the spectrum
    let spectrum = 0..10;
    println!("The spectrum is: ");
    prints::print_spectrum();

    // Draw card
    let path = Path::new("cards.txt");

    let file = File::open(path);
    let mut lines = Vec::new();
    for line in io::BufReader::new(file.unwrap()).lines() {
        lines.push(line.unwrap());
    }

    let mut total_points = 0;
    let n_rounds = 7;
    for round in 1..n_rounds + 1 {
        println!("\nPress enter to start round {}.", round);
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        println!("{}###################################### ROUND {round} ########################################", COLORMAP[round - 1]);
        println!("{}", RESET);

        let mut random_line ;
        loop {
            answer.clear();
            let mut rng = rand::rng();
            let random_index = rng.random_range(0..lines.len());
            random_line = &lines[random_index];
            println!("You draw the following card: {:?}", random_line);

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

        println!("The card is: {:?}", random_line);

        // The Psychic gives a clue
        println!("Psychic, please give a clue (e.g., a word or phrase):");
        let mut clue = String::new();
        io::stdin().read_line(&mut clue).expect("Failed to read line");

        // The Guesser tries to guess the position
        println!("Guesser, please guess the position on the spectrum ({}, {}):", spectrum.start, spectrum.end);
        prints::print_spectrum();

        let guess: i32
            ;
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<i32>() {
                Ok(num) if num <= 10 => {
                    guess = num;
                    break;
                }
                Ok(_) => { println!("The number must be between 0 and 10.");}
                Err(_) => { println!("Please enter a valid number."); }
            }
        }

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
