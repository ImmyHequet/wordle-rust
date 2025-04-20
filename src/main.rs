mod word_handler;
use crate::word_handler::{check_guess, select_random_word};

fn main() {
    let word = select_random_word().unwrap();
    let guess = "apple";

    let result = check_guess(&guess, &word);

    println!("target word: {}. Guess word: {}", word, guess);
    println!("Result: {:?}", result);
}
