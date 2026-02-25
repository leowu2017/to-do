use colored::Colorize;
use rand;
use std::{fs, io};

const FILE_NAME: &str = "scores.txt";

fn load_high_scores() -> Vec<u32> {
    match fs::read_to_string(FILE_NAME) {
        Ok(scores) => {
            match scores
                .split_whitespace()
                .map(|s| s.parse::<u32>())
                .collect()
            {
                Ok(scores) => scores,
                Err(_) => Vec::new(),
            }
        }
        Err(_) => Vec::new(),
    }
}

fn update_high_scores(score: u32, high_scores: &mut Vec<u32>) {
    let max_high_score_num: usize = 3;
    let mut updated = false;
    for (idx, val) in high_scores.iter().enumerate() {
        if score <= *val {
            high_scores.insert(idx, score);
            updated = true;
            break;
        }
    }
    if !updated {
        high_scores.push(score);
    }
    if high_scores.len() > max_high_score_num {
        high_scores.pop();
    }
}

fn print_high_scores(high_scores: &Vec<u32>) {
    let mut msg = String::from("High Scores:\n");
    for (idx, val) in high_scores.iter().enumerate() {
        msg.push_str(&format!("{}. {}\n", idx + 1, val));
    }
    println!("{}", msg.blue());
}

fn save_high_scores(high_scores: &Vec<u32>) {
    let output = high_scores
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    match fs::write(FILE_NAME, output) {
        Ok(_) => (),
        Err(e) => println!("Failed to save file: {e}"),
    }
}

fn main() {
    // Generate a target number.
    let target_from: i32 = 0;
    let target_to: i32 = 10;
    let target_number: i32 = rand::random_range(target_from..=target_to);
    let target_attempt: u32 = 10;

    // Load high scores.
    let mut high_scores: Vec<u32> = load_high_scores();

    // loop until the user successfully guess the correct target.
    let mut attempt: u32 = 1;
    loop {
        // Let user guess a number.
        println!("Guess a number between {target_from} and {target_to}.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");
        let number: i32 = input.trim().parse().expect("Filed to convert to number!");
        if number > target_number {
            println!("Incorrect! The target is lower.");
        } else if number < target_number {
            println!("Incorrect! The target is higher.");
        } else {
            println!("{}", "Congrats! You guess the correct number!".green());
            println!("Your score: {attempt}");
            // Update and print high scores.
            update_high_scores(attempt, &mut high_scores);
            print_high_scores(&high_scores);
            save_high_scores(&high_scores);
            break;
        }

        // out of attempts
        attempt += 1;
        if attempt > target_attempt {
            println!("{}", "Failed! You are out of attempts.".red());
            break;
        }
    }
}
