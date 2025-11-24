use std::process::exit;
mod compute_sus;
mod context;
mod builder;

use context::WORDLE_WIDTH;

fn print_help(app_name: &str) {
    println!("Finds the 6 words to use in today's Wordle to obtain a \"SUS\" looking board.");
    println!("Usage: {app_name} <word> [OPTIONS]");
    println!("The word must be exactly {WORDLE_WIDTH} characters long.");
    println!("\nOptions:");
    println!("  --help       Show this help message");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No arguments provided.");
        print_help(&args[0]);
        exit(1);
    }

    if args.contains(&String::from("--help")) {
        print_help(&args[0]);
        return;
    }

    let word = &args[1].to_lowercase();
    let letters: [char; WORDLE_WIDTH] = match word.chars().collect::<Vec<char>>().try_into() {
        Ok(array) => array,
        Err(_) => {
            eprintln!("Error: The word must be exactly {WORDLE_WIDTH} characters long.");
            exit(2);
        }
    };

    let results = compute_sus::compute_sus(&letters);
    let results = match results {
        None => {
            println!("No solution found for the word \"{}\".", word);
            exit(3);
        }
        Some(res) => {
            res
        },
    };

    let mut skip = true;
    for result in results.iter() {
        if skip && result == word {
            println!("The found solution have a green background; we skip the first line as it matches the input word.");
            skip = false;
            continue;
        }
        println!("{}", result);
    }
}
