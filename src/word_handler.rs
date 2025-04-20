use log::{error, info};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

//  function to read and filter dictionary to the 5 letter words
//  Select a random word

pub fn get_all_5_letter_words() -> Result<Vec<String>, io::Error> {
    let path = "/usr/share/dict/words";
    let mut five_letter_words = Vec::new();

    if let Ok(file) = File::open(&path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(word) = line {
                let trimmed_word = word.trim();
                if trimmed_word.len() == 5 {
                    five_letter_words.push(trimmed_word.to_lowercase());
                }
            } else {
                error!("Error reading line from file");
            }
        }
    } else {
        error!("Failed to open the dictionary file at {}", path);
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Dictionary file not found",
        ));
    }
    info!("Found {} five-letter words", five_letter_words.len());
    Ok(five_letter_words)
}
