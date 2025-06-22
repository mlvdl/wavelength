use std::fs::File;
use rand::Rng;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::process::Command;


// Define ANSI color codes
const RESET: &str = "\x1B[0m";
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
const YELLOW: &str = "\x1B[33m";
const BLUE: &str = "\x1B[34m";
const MAGENTA: &str = "\x1B[35m";
const CYAN: &str = "\x1B[36m";
const _WHITE: &str = "\x1B[37m";


const COLORMAP: [&str; 8] = [
    "\x1b[37m", // White
    "\x1b[31m", // Red
    "\x1b[33m", // Yellow
    "\x1b[32m", // Green
    "\x1b[36m", // Cyan
    "\x1b[34m", // Blue
    "\x1b[35m", // Magenta
    "\x1b[30m", // Black
];


fn main() {
    println!("Welcome to the digital cooperative version of Wavelength!");
    print_banner();
    println!("How to play:");
    println!("The objective of Wavelength is to give your teammates a clue allowing them to accurately predict where to target on a spectrum. \
    \nIf you get more than 15 points, you win!");

    println!("Press (h) if you need more help, enter to continue.");
    let mut help = String::new();
    io::stdin().read_line(&mut help).expect("Failed to read line");
    if help.starts_with('h') {
        print_help();
    }

    // Define the spectrum
    let spectrum = 0..1;
    println!("The spectrum is: ");
    print_spectrum();

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
        let mut answer = String::new();
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
        clear_terminal();

        println!("The card is: {:?}", random_line);

        // The Psychic gives a clue
        println!("Psychic, please give a clue (e.g., a word or phrase):");
        let mut clue = String::new();
        io::stdin().read_line(&mut clue).expect("Failed to read line");

        // The Guesser tries to guess the position
        println!("Guesser, please guess the position on the spectrum ({}, {}):", spectrum.start, spectrum.end);
        print_spectrum();

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

    if total_points > 15 { print_win(); }
    else { print_lose();}
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
    print_banner();
}

fn print_banner() {
    println!("{}                                 _                  _   _              ", RED);
    println!("{}        __      ____ ___   _____| | ___ _ __   __ _| |_| |__   ", YELLOW);
    println!("{}        \\ \\ /\\ / / _` \\ \\ / / _ \\ |/ _ \\ '_ \\ / _` | __| '_ \\  ", GREEN);
    println!("{}         \\ V  V / (_| |\\ V /  __/ |  __/ | | | (_| | |_| | | | ", CYAN);
    println!("{}          \\_/\\_/ \\__,_| \\_/ \\___|_|\\___|_| |_|\\__, |\\__|_| |_| ", BLUE);
    println!("{}                                              |___/                        ", MAGENTA);
    println!("{}", RESET);
}

fn print_help() {
    println!("Wavelength is played over rounds, where the teammates will alternate playing the psychic. Each round consists of 3 phases.");
    println!("1. Psychic Phase.");
    println!("2. Team Phase.");
    println!("3. Scoring Phase.");
    println!("");
    println!("1. Psychic draws a card. Wavelength cards list 2 opposite ends of a spectrum.");
    println!("Psychic gets to see where the hidden target stands, and gives a clue for the target area's position.");
    println!("The clue can be up to 5 words, and should relate to the spectrum but be abstract enough to challenge the team.");
    println!("2. Teammates try to guess where the hidden target is in the spectrum.");
    println!("3. Scores: 4 point if the target position is guessed, 3 points for +/-1, 2 points for +/-2, 0 otherwise.");
    println!("\nGood luck!\n");
}

fn print_spectrum() {
    print!("{} 0", RED);
    print!("{} 1", YELLOW);
    print!("{} 2", YELLOW);
    print!("{} 3", GREEN);
    print!("{} 4", GREEN);
    print!("{} 5", CYAN);
    print!("{} 6", CYAN);
    print!("{} 7", BLUE);
    print!("{} 8", BLUE);
    print!("{} 9", MAGENTA);
    println!("{}", RESET);
}

fn print_lose() {
    print!("{} ", RED);
    println!("            __   _____  _   _   _     ___  ____  _____  ");
    println!("             \\ \\ / / _ \\| | | | | |   / _ \\/ ___|| ____|");
    println!("              \\ V / | | | | | | | |  | | | \\___ \\|  _|");
    println!("               | || |_| | |_| | | |__| |_| |___) | |___");
    println!("               |_| \\___/ \\___/  |_____\\___/|____/|_____|");
    println!();
}

fn print_win() {
    print!("{} ", GREEN);
    println!();
    println!("             __   _____  _   _  __        _____ _   _  ");
    println!("             \\ \\ / / _ \\| | | | \\ \\      / /_ _| \\ | | ");
    println!("              \\ V / | | | | | |  \\ \\ /\\ / / | ||  \\| |");
    println!("               | || |_| | |_| |   \\ V  V /  | || |\\  |");
    println!("               |_| \\___/ \\___/     \\_/\\_/  |___|_| \\_|");
    println!();
}