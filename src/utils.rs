use std::process::Command;
use std::io;
use crate::prints;

pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
    prints::print_banner();
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn rgb_to_ansi(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

pub fn get_color(value: i32, max_value: i32) -> String {
    if max_value == 0 {
        return "\x1b[0m".to_string();
    }
    let hue = (value as f32 / max_value as f32) * 300.0;
    let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);
    rgb_to_ansi(r, g, b)
}

pub fn read_number(min: i32, max: i32) -> i32 {
    let guess: i32;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<i32>() {
            Ok(num) if (min <= num) & (num <= max) => {
                guess = num;
                break;
            }
            Ok(_) => { println!("The number must be between {} and {}.", min, max); }
            Err(_) => { println!("Please enter a valid number."); }
        }
    }
    guess
}