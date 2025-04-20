use log::{error, info};
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

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
