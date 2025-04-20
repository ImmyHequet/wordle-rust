mod word_handler;
use crate::word_handler::get_all_5_letter_words;

fn main() {
    let words = get_all_5_letter_words();
    println!("Total 5-letter words: {}", words.unwrap().len());
    // println!("Sample words:");
    // for word in words.iter().take(10) {
    //     println!("{}", word);
    // }
}
