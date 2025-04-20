mod word_handler;
use crate::word_handler::select_random_word;

fn main() {
    let word = select_random_word().unwrap();

    println!("a word: {}", word);
}
