use log::{error, info};
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, PartialEq, Clone)]
pub enum LetterResult {
    Correct,   // Green - right letter, right position
    Misplaced, // Yellow - right letter, wrong position
    Wrong,     // Gray - letter not in word
}

fn load_words(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        if let Ok(word) = line {
            words.push(word.trim().to_string());
        }
    }

    if words.is_empty() {
        error!("No words found in the file.");
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No words found in the file",
        ))
    } else {
        Ok(words)
    }
}

fn pick_random_word(words: &[String]) -> String {
    let random_index = rand::thread_rng().gen_range(0..words.len());
    let selected_word = &words[random_index];
    info!("Randomly selected word: {}", selected_word);
    selected_word.clone()
}

pub fn select_random_word() -> Result<String, io::Error> {
    let path = "./resources/wordle_words.txt";

    match load_words(path) {
        Ok(words) => Ok(pick_random_word(&words)),
        Err(e) => {
            error!("Failed to load words: {}", e);
            Err(e)
        }
    }
}

pub fn check_guess(guess: &str, target: &str) -> Result<Vec<LetterResult>, String> {
    if guess.len() != target.len() {
        return Err(format!(
            "Guess length {} does not match target length {}",
            guess.len(),
            target.len()
        ));
    }

    let mut result = vec![LetterResult::Wrong; guess.len()];
    let mut target_chars = target.chars().collect::<Vec<_>>();
    let mut guess_chars = guess.chars().collect::<Vec<_>>();

    // First pass: Check for correct letters in the correct position
    for i in 0..guess.len() {
        if guess_chars[i] == target_chars[i] {
            result[i] = LetterResult::Correct;
            target_chars[i] = ' '; // Mark as used
            guess_chars[i] = ' '; // Mark as used
        }
    }

    // Second pass: Check for misplaced letters
    for i in 0..guess.len() {
        if result[i] == LetterResult::Wrong && guess_chars[i] != ' ' {
            let current_guess_char = guess_chars[i];

            let maybe_postion = target_chars.iter().position(|&c| c == current_guess_char);

            if let Some(pos) = maybe_postion {
                result[i] = LetterResult::Misplaced;
                target_chars[pos] = ' '; // Mark as used
                guess_chars[i] = ' '; // Mark as used
            }
        }
    }

    Ok(result)
}

// first pass
