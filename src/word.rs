use std::fmt::{Display, Formatter, Result};

/// Represents a letter in the game.
///
/// # Example
/// ```
/// let letter = Letter {
///   character: 'h',
///   guessed: false,
/// };
/// assert_eq!('h', letter.character);
/// assert_eq!(false, letter.guessed);
/// ```
#[derive(Debug)]
struct Letter {
    character: char,
    guessed: bool,
}

/// Represents a word to be found.
#[derive(Debug)]
pub struct Word(Vec<Letter>);

/// The result of a guess.
///
/// # Example
/// ```
/// let word = Word::from("hello");
/// let result = word.guess('h');
/// assert_eq!(true, result.was_correct);
/// assert_eq!("h____", result.new_word.to_string());
/// // The original Word is not changed.
/// assert_eq!(word, Word::from("hello"));
/// ```
pub struct GuessResult {
    /// The [`Word`] that the guess was made on, with any correctly guessed letters revealed
    ///
    /// [`Word`]: struct.Word.html
    pub new_word: Word,
    /// `true` if the guess revealed any new letters, `false` otherwise.
    pub was_correct: bool,
}

impl Word {
    /// `true` if all letters have been guessed, `false` otherwise.
    pub fn is_guessed(&self) -> bool {
        self.0.iter().all(|l| l.guessed)
    }

    /// Performs a guess on this word, returning the [`GuessResult`]
    ///
    /// [`GuessResult`]: struct.GuessResult.html
    pub fn guess(&self, c: char) -> GuessResult {
        let new_letters = self
            .0
            .iter()
            .map(|l| Letter {
                guessed: l.guessed || l.character == c,
                ..*l
            })
            .collect();
        let new_word = Word(new_letters);
        let was_correct = self.0.iter().any(|l| l.character == c);
        GuessResult {
            new_word,
            was_correct,
        }
    }

    /// Returns a [`String`] containing the word with all letters shown.
    ///
    /// [`String`]: /alloc/string/struct.String.html
    pub fn revealed(&self) -> String {
        self.0.iter().map(|c| c.character).collect()
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", if self.guessed { self.character } else { '_' })
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for x in &self.0 {
            write!(f, "{}", x)?;
        }
        Result::Ok(())
    }
}

impl From<&str> for Word {
    fn from(string: &str) -> Word {
        Word(
            string
                .chars()
                .map(|c| Letter {
                    character: c,
                    guessed: false,
                })
                .collect(),
        )
    }
}
