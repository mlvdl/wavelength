pub mod competitive;
pub mod cooperative;

pub(crate) trait GameMode {
    fn play(&mut self);
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
