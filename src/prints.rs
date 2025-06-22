use crate::colors::{BLUE, CYAN, GREEN, MAGENTA, RED, RESET, YELLOW};
use crate::utils;

pub fn print_lose() {
    print!("{} ", RED);
    println!("            __   _____  _   _   _     ___  ____  _____  ");
    println!("             \\ \\ / / _ \\| | | | | |   / _ \\/ ___|| ____|");
    println!("              \\ V / | | | | | | | |  | | | \\___ \\|  _|");
    println!("               | || |_| | |_| | | |__| |_| |___) | |___");
    println!("               |_| \\___/ \\___/  |_____\\___/|____/|_____|");
    println!();
}

pub fn print_win() {
    print!("{} ", GREEN);
    println!();
    println!("             __   _____  _   _  __        _____ _   _  ");
    println!("             \\ \\ / / _ \\| | | | \\ \\      / /_ _| \\ | | ");
    println!("              \\ V / | | | | | |  \\ \\ /\\ / / | ||  \\| |");
    println!("               | || |_| | |_| |   \\ V  V /  | || |\\  |");
    println!("               |_| \\___/ \\___/     \\_/\\_/  |___|_| \\_|");
    println!();
}

pub fn print_banner() {
    println!("{}                                 _                  _   _               ", RED);
    println!("{}        __      ____ ___   _____| | ___ _ __   __ _| |_| |__            " , YELLOW);
    println!("{}        \\ \\ /\\ / / _` \\ \\ / / _ \\ |/ _ \\ '_ \\ / _` | __| '_ \\  ", GREEN);
    println!("{}         \\ V  V / (_| |\\ V /  __/ |  __/ | | | (_| | |_| | | |        ", CYAN);
    println!("{}          \\_/\\_/ \\__,_| \\_/ \\___|_|\\___|_| |_|\\__, |\\__|_| |_|  ", BLUE);
    println!("{}                                              |___/                     ", MAGENTA);
    println!("{}", RESET);
}

pub fn print_help() {
    println!("Wavelength is played over rounds, where the teammates will alternate playing the psychic. Each round consists of 3 phases.");
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
    for value in start..=end {
        let color = utils::get_color(value - start, end - start);
        print!("{} {} {}", color, value, "\x1b[0m");
    }
    println!();
}

pub fn print_card(content: &str, color_code: &String) {
    let lines: Vec<&str> = content.split('\n').collect();
    let max_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let horizontal_border = format!("+{}+", "-".repeat(max_length + 2));

    println!("\n{}                {}", color_code, horizontal_border);
    for line in lines {
        println!("                | \x1B[1m{: <width$}\x1B[0m {}|", line, color_code, width = max_length);
    }
    println!("                {}{}", color_code, horizontal_border);
    println!("{}", RESET);
}

pub fn print_round_banner(round: usize, color_code: &String) {
    println!("{}###################################### ROUND {round} ########################################", color_code);
    println!("{}", RESET);
}