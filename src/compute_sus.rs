use crate::builder::SusBuilder;
use crate::context::{Color, WORDLE_WIDTH};
use crate::context::{fill_context, get_colors};
use std::collections::HashMap;

const FILE: &str = include_str!("words");

/// Search for the first possible way to draw a crewmate (from the game Among Us) in a Wordle board.
/// Considering the input word as the word to be guessed, we have 3 possible colors for each letter:
/// - Gray: the letter is not in the word
/// - Yellow: the letter is in the word but in a different position
/// - Green: the letter is in the word and in the correct position
/// This function will return 6 words that, when used as guesses in Wordle, will create a pattern
/// that resembles a crewmate.
///
/// The crewmate pattern is defined as follows:
/// .....
/// .###.
/// .**##
/// .####
/// .#.#.
/// .....
///
/// Note that any f the 3 colors can be used for any of the symbols (., #, *).
/// The pattern can also be mirrored horizontally.
/// The pattern also can be shifted left or right within the 5-letter width of the Wordle board.
///
/// # Arguments
///
/// * `word`: Today's Wordle word as an array of 5 characters
///
/// returns: [String; 6]
///
/// # Examples
///
/// ```
/// TODO
/// ```
pub fn compute_sus(word: &[char; WORDLE_WIDTH]) -> Option<[String; 6]> {
    let words: Vec<&str> = FILE.lines().collect();

    let mut map: HashMap<Color, SusBuilder> = HashMap::new();

    for candidate in words {
        let letters: [char; WORDLE_WIDTH] =
            match candidate.chars().collect::<Vec<char>>().try_into() {
                Ok(array) => array,
                Err(_) => continue,
            };

        let colors = get_colors(word, &letters);
        let builder = fill_context(&mut map, &colors, candidate);
        if let Some(builder) = builder {
            if builder.is_full() {
                return Some(builder.convert());
            }
        }
    }

    None
}
