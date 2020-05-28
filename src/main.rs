extern crate eff_wordlist;

mod word;

use crate::word::Word;
use eff_wordlist::large::random_word;
use std::io::{stdin, stdout, BufRead, Write};

const MAX_INCORRECT_GUESSES: i32 = 8;

/// Plays the game.
///
/// Picks a random word and prompts the user to guess letters until either:
/// * They have revealed all letters in the word, in which case they win
/// * They have run out of guesses, in which case they lose
fn main() {
    let mut word = Word::from(random_word());
    println!("{}", word);

    let mut incorrect_guesses = 0;

    let mut buffer = String::new();

    while !word.is_guessed() && incorrect_guesses < MAX_INCORRECT_GUESSES {
        print!("Pick a letter to guess: ");
        stdout().flush().unwrap();

        stdin().lock().read_line(&mut buffer).unwrap();

        let guess = buffer.chars().next().unwrap_or(' ');

        buffer.clear();

        let result = word.guess(guess);

        word = result.new_word;
        if !result.was_correct {
            incorrect_guesses += 1;
        }

        println!("{}", word);
        println!("{} incorrect guesses.", incorrect_guesses);
        println!();
    }

    if incorrect_guesses == MAX_INCORRECT_GUESSES {
        println!(
            "You didn't guess the word, which was '{}'. The man hanged.",
            word.revealed()
        );
    } else {
        println!("🚀 You guessed the word, and saved the man!");
    }
}
