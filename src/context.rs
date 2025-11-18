use crate::builder::SusBuilder;
use crate::context::Color::Green;
use std::collections::HashMap;

pub const WORDLE_WIDTH: usize = 5;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    Gray,
    Yellow,
    Green,
}

#[derive(Hash, PartialEq, Eq, Clone)]
enum Direction {
    LookingLeft,
    LookingRight,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Shift {
    HuggingLeft,
    HuggingRight,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Context {
    body: Color,
    direction: Direction,
    shift: Shift,
}

impl Color {
    fn other(&self, second: &Color) -> Color {
        match (self, second) {
            (Color::Gray, Color::Yellow) | (Color::Yellow, Color::Gray) => Color::Green,
            (Color::Gray, Color::Green) | (Color::Green, Color::Gray) => Color::Yellow,
            _ => Color::Gray,
        }
    }
}

pub fn get_colors(
    solution: &[char; WORDLE_WIDTH],
    guess: &[char; WORDLE_WIDTH],
) -> [Color; WORDLE_WIDTH] {
    let mut colors = [Color::Gray; WORDLE_WIDTH];
    let mut solution_used = [false; WORDLE_WIDTH];

    // First pass: check for greens
    for i in 0..WORDLE_WIDTH {
        if guess[i] == solution[i] {
            colors[i] = Color::Green;
            solution_used[i] = true;
        }
    }

    // Second pass: check for yellows
    for i in 0..WORDLE_WIDTH {
        if colors[i] != Color::Gray {
            continue;
        }

        for j in 0..WORDLE_WIDTH {
            if guess[i] == solution[j] && !solution_used[j] {
                colors[i] = Color::Yellow;
                solution_used[j] = true;
                break;
            }
        }
    }

    colors
}

/// Fills the context map with the given colors and word.
/// The colors can be Gray, Yellow or Green.
/// The context is determined by the colors of the background and body, the shift (left or right),
/// and the direction (left or right).
/// If the SusBuilder for the given context is not complete, it is updated with the given word.
/// If the SusBuilder for the given context is complete, it is returned.
/// If the SusBuilder for a given context does not exist, it is created.
///
/// This function also determines the pattern for this color combination, and updates the context accordingly.
/// The pattern is defined as follows:
/// - Empty: only one color in the entire word
/// AAAAA
/// - Head: three consecutive colors, the other two colors are the same
/// BBBAA or AABBB or ABBBA
/// - Eyes: two consecutive colors, followed by a second two consecutive colors, then the last color
/// BBCCA or CCBBA or ACCBB or ABBCC
/// - Body: four consecutive colors, the other color is different
/// BBBBA or ABBBB
/// - Legs: Two colors A separated by one color B, any other color is B
/// BABAA or AABAB or ABABA
/// # Arguments
///
/// * `map`: map of Context to SusBuilder
/// * `colors`: array of colors for the given word
/// * `word`: the word to be added to the SusBuilder
///
/// returns: Option<SusBuilder>
pub fn fill_context<'a, 'b>(
    map: &'b mut HashMap<Color, SusBuilder<'a>>,
    colors: &[Color; WORDLE_WIDTH],
    word: &'a str,
) -> Option<&'b SusBuilder<'a>> {
    let color_count = count_colors(colors);

    if color_count == 1 {
        // one color means it's an empty line
        if colors[0] == Green {
            None // we disallow green background
        } else {
            handle_empty_line(map, colors, word)
        }
    } else if color_count == 3 {
        // there are three colors, so it's the eyes or an invalid pattern
        if let Some((left_color, right_color, shift)) = is_valid_eyes(colors) {
            handle_eyes_pattern(map, word, left_color, right_color, shift)
        } else {
            None
        }
    }
    // here, we know there are exactly 2 colors, so it can be any of the other patterns
    else if let Some((background, body, shift)) = is_four_in_a_row(colors) {
        handle_body_pattern(map, word, background, body, shift)
    } else if let Some((background, body, shift)) = is_three_in_a_row(colors) {
        handle_head_pattern(map, word, background, body, shift)
    } else if let Some((background, body, shift)) = is_there_two_separated(colors) {
        handle_leg_pattern(map, word, background, body, shift)
    } else {
        None // no matching pattern found for this word
    }
}

fn handle_empty_line<'a, 'b>(
    map: &'b mut HashMap<Color, SusBuilder<'a>>,
    colors: &[Color; 5],
    word: &'a str,
) -> Option<&'b SusBuilder<'a>> {
    let background = colors[0];
    let builder = map.entry(background).or_default();
    if builder.empty.is_none() {
        builder.empty = Some(word);
    }

    if builder.is_full() {
        Some(builder)
    } else {
        None
    }
}

fn handle_body_pattern<'a, 'b>(
    map: &'b mut HashMap<Color, SusBuilder<'a>>,
    word: &'a str,
    background: Color,
    body: Color,
    shift: Shift,
) -> Option<&'b SusBuilder<'a>> {
    let builder = map.entry(background).or_default();

    builder.fill_body(
        Context {
            body,
            direction: Direction::LookingRight,
            shift: shift.clone(),
        },
        word,
    );

    builder.fill_body(
        Context {
            body,
            direction: Direction::LookingLeft,
            shift,
        },
        word,
    );

    if builder.is_full() {
        Some(builder)
    } else {
        None
    }
}

fn handle_head_pattern<'a, 'b>(
    map: &'b mut HashMap<Color, SusBuilder<'a>>,
    word: &'a str,
    background: Color,
    body: Color,
    shift: Option<Shift>,
) -> Option<&'b SusBuilder<'a>> {
    let builder = map.entry(background).or_default();

    match shift {
        None => handle_head_middle_pattern(builder, word, body),
        Some(Shift::HuggingLeft) => {
            builder.fill_head(
                Context {
                    body,
                    direction: Direction::LookingLeft,
                    shift: Shift::HuggingLeft,
                },
                word,
            );
        }
        Some(Shift::HuggingRight) => {
            builder.fill_head(
                Context {
                    body,
                    direction: Direction::LookingRight,
                    shift: Shift::HuggingRight,
                },
                word,
            );
        }
    }

    if builder.is_full() {
        Some(builder)
    } else {
        None
    }
}

fn handle_head_middle_pattern<'a, 'b>(builder: &'b mut SusBuilder<'a>, word: &'a str, body: Color) {
    builder.fill_head(
        Context {
            body,
            direction: Direction::LookingLeft,
            shift: Shift::HuggingRight,
        },
        word,
    );

    builder.fill_head(
        Context {
            body,
            direction: Direction::LookingRight,
            shift: Shift::HuggingLeft,
        },
        word,
    );
}

fn handle_leg_pattern<'a, 'b>(
    map: &'b mut HashMap<Color, SusBuilder<'a>>,
    word: &'a str,
    background: Color,
    body: Color,
    shift: Option<Shift>,
) -> Option<&'b SusBuilder<'a>> {
    let builder = map.entry(background).or_default();

    match shift {
        None => handle_leg_middle_pattern(builder, word, body),
        Some(Shift::HuggingLeft) => {
            builder.fill_legs(
                Context {
                    body,
                    direction: Direction::LookingLeft,
                    shift: Shift::HuggingLeft,
                },
                word,
            );
        }
        Some(Shift::HuggingRight) => {
            builder.fill_legs(
                Context {
                    body,
                    direction: Direction::LookingRight,
                    shift: Shift::HuggingRight,
                },
                word,
            );
        }
    }

    if builder.is_full() {
        Some(builder)
    } else {
        None
    }
}

fn handle_leg_middle_pattern<'a, 'b>(builder: &'b mut SusBuilder<'a>, word: &'a str, body: Color) {
    builder.fill_legs(
        Context {
            body,
            direction: Direction::LookingLeft,
            shift: Shift::HuggingRight,
        },
        word,
    );

    builder.fill_legs(
        Context {
            body,
            direction: Direction::LookingRight,
            shift: Shift::HuggingLeft,
        },
        word,
    );
}

fn handle_eyes_pattern<'a, 'b>(
    map: &'b mut HashMap<Color, SusBuilder<'a>>,
    word: &'a str,
    left_color: Color,
    right_color: Color,
    shift: Shift,
) -> Option<&'b SusBuilder<'a>> {
    let background = left_color.other(&right_color);
    let builder = map.entry(background).or_default();

    builder.fill_eyes(
        Context {
            body: left_color,
            direction: Direction::LookingRight,
            shift: shift.clone(),
        },
        word,
    );
    builder.fill_eyes(
        Context {
            body: right_color,
            direction: Direction::LookingLeft,
            shift: shift.clone(),
        },
        word,
    );

    if builder.is_full() {
        Some(builder)
    } else {
        None
    }
}

fn count_colors(colors: &[Color; WORDLE_WIDTH]) -> usize {
    let mut unique_colors = std::collections::HashSet::new();
    for &color in colors.iter() {
        unique_colors.insert(color);
    }
    unique_colors.len()
}

/// Test if the lines contain four in a row of the same color.
/// Returns the two colors and the shift if found.
///
/// # Arguments
///
/// * `colors`: line
///
/// returns: None if no four in a row is found, or Some((Color, Color, Shift)) if found.
///
/// First Color is background color, second is body color.
///
/// The shift is HuggingLeft if the four in a row starts at index 0,
/// HuggingRight if it starts at index 1.
fn is_four_in_a_row(colors: &[Color; WORDLE_WIDTH]) -> Option<(Color, Color, Shift)> {
    if colors[0] == colors[1] && colors[1] == colors[2] && colors[2] == colors[3] {
        let body = colors[0];
        let background = colors[4];
        Some((background, body, Shift::HuggingLeft))
    } else if colors[1] == colors[2] && colors[2] == colors[3] && colors[3] == colors[4] {
        let body = colors[1];
        let background = colors[0];
        Some((background, body, Shift::HuggingRight))
    } else {
        None
    }
}

/// Checks if there are three in a row of the same color in the given colors array.
/// The two remaining colors must be the same color as well.
///
/// Three patterns are possible:
/// - AAABB
/// - BAAAB
/// - BBAAA
///
/// Where A is the body color and B is the background color.
///
/// # Arguments
///
/// * `colors`: All values can't be the same color.
///
/// returns: Option<(Color, Color, Option<Shift>)>
///
/// First Color is background color, second is body color.
/// The Shift is HuggingLeft if the three in a row starts at index 0,
///
/// HuggingRight if it starts at index 2,
///
/// None if it starts at index 1.
fn is_three_in_a_row(colors: &[Color; WORDLE_WIDTH]) -> Option<(Color, Color, Option<Shift>)> {
    if colors[0] == colors[1] && colors[1] == colors[2] && colors[3] == colors[4] {
        Some((colors[3], colors[0], Some(Shift::HuggingLeft)))
    } else if colors[1] == colors[2] && colors[2] == colors[3] && colors[0] == colors[4] {
        Some((colors[0], colors[1], None))
    } else if colors[2] == colors[3] && colors[3] == colors[4] && colors[0] == colors[1] {
        Some((colors[0], colors[2], Some(Shift::HuggingRight)))
    } else {
        None
    }
}

/// Checks if there are two separated equal colors in the given colors array.
/// The three remaining colors must be the same color.
///
/// Three patterns are possible:
/// - ABABB
/// - BABAB
/// - BBABA
///
/// Where A is the body color and B is the background color.
///
/// # Arguments
///
/// * `colors`: All values can't be the same color.
///
/// returns: Option<(Color, Color, Option<Shift>)>
///
/// First Color is background color, second is body color.
///
/// The Shift is HuggingLeft if the pattern starts at index 0,
///
/// HuggingRight if it starts at index 2,
///
/// None if it starts at index 1.

fn is_there_two_separated(colors: &[Color; WORDLE_WIDTH]) -> Option<(Color, Color, Option<Shift>)> {
    if colors[0] == colors[2] && colors[1] == colors[3] && colors[1] == colors[4] {
        Some((colors[1], colors[0], Some(Shift::HuggingLeft)))
    } else if colors[1] == colors[3] && colors[0] == colors[2] && colors[0] == colors[4] {
        Some((colors[0], colors[1], None))
    } else if colors[2] == colors[4] && colors[0] == colors[1] && colors[0] == colors[3] {
        Some((colors[0], colors[2], Some(Shift::HuggingRight)))
    } else {
        None
    }
}

/// Assuming that we know there are exactly 3 colors in the given colors array,
/// checks if they can form the "eyes" pattern.
///
/// The "Eyes" pattern is defined as two consecutive colors, followed by another two consecutive colors,
/// and a third color at the end or beginning.
/// - CCAAB
/// - AACCB
/// - BAACC
/// - BCCAA
///
/// Where A is the body color, B is the background color, and C is the eye color.
///
/// # Arguments
///
/// * `colors`:
///
/// returns: Option<(Color, Color, Option<Shift>)>
///
/// The two colors are background and body colors in order of appearance.
///
/// The Shift is HuggingLeft if the pattern starts at index 0,
/// HuggingRight if it starts at index 1,
fn is_valid_eyes(colors: &[Color; WORDLE_WIDTH]) -> Option<(Color, Color, Shift)> {
    if colors[0] == colors[1] && colors[2] == colors[3] {
        Some((colors[0], colors[2], Shift::HuggingLeft))
    } else if colors[1] == colors[2] && colors[3] == colors[4] {
        Some((colors[1], colors[3], Shift::HuggingRight))
    } else {
        None
    }
}
