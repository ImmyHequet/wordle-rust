mod word_handler;
use crate::word_handler::{check_guess, select_random_word, LetterResult};
use colored::*;
use std::io::{self, Write};

fn get_user_guess() -> String {
    let mut input = String::new();

    print!("Enter your guess: ");

    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn is_valid_guess(guess: &str) -> bool {
    guess.len() == 5 && guess.chars().all(|c| c.is_alphabetic())
}

fn display_result(guess: &str, result: &[LetterResult]) {
    for (i, letter) in guess.chars().enumerate() {
        let colored_letter = match result[i] {
            LetterResult::Correct => letter.to_string().white().on_green(),
            LetterResult::Misplaced => letter.to_string().white().on_yellow(),
            LetterResult::Wrong => letter.to_string().white().on_bright_black(),
        };
        print!("{} ", colored_letter);
    }
    println!();
}

fn main() {
    println!("Welcome to Rust Wordle!");
    println!("Try to guess the 5-letter word in 6 tries.");
    println!("After each guess, you'll get feedback:");
    println!("- Green: Correct letter in correct position");
    println!("- Yellow: Correct letter in wrong position");
    println!("- White/Gray: Letter not in the word");

    let target_word = select_random_word().unwrap();

    let max_attempts = 6;
    let mut attempts = 0;
    let mut game_won = false;

    while attempts < max_attempts && !game_won {
        println!("Attempt:{}", attempts);

        let guess = get_user_guess();

        if !is_valid_guess(&guess) {
            println!("Invalid guess! Please enter a 5-letter word using only letters.");
            continue; // Skip this iteration, don't count as an attempt
        }

        let result = check_guess(&guess, &target_word).unwrap();

        display_result(&guess, &result);

        if result.iter().all(|r| *r == LetterResult::Correct) {
            game_won = true;
            println!("\nCongratulations! You guessed the word: {}", target_word);
        }

        attempts += 1;
    }
    if !game_won {
        println!("\nGame over! The word was: {}", target_word);
    }
}
