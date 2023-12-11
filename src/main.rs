mod wordle;

use std::{io::{BufReader, BufRead, Write}, env, fs::File, collections::HashSet, iter::zip};
use rand::{self, thread_rng, Rng};
use anyhow::Result;
use console::Term;

use wordle::Game;

pub fn load_word_list() -> Result<HashSet<String>> {
    let mut file_path = env::current_dir()?;
    file_path.push("word_list");

    let file = File::open(file_path)?;
    let buf = BufReader::new(file);
    Ok(buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect())
  }

fn main() -> Result<()> {
    let word_list = load_word_list()?;

    let random_word_index: usize = thread_rng().gen_range(0..word_list.len());
    let answer_str = word_list.iter().skip(random_word_index).next().unwrap();
    let answer_arr: [char; 5] = answer_str
        .chars().into_iter()
        .collect::<Vec<char>>().try_into().unwrap();

    let game = Game::new(answer_arr);
    let mut term = Term::stdout();

    term.write_line("Welcome to wordle")?;

    let mut guess_idx = 0;

    let mut result_history: [String; 6] = core::array::from_fn(|_| "⬛⬛⬛⬛⬛".to_owned());
    let mut guess_history: [String; 6] = core::array::from_fn(|_| "     ".to_owned());
    let mut message_to_user = String::new();
    let mut won: bool = false;
    while guess_idx < 6 {
        // render game
        for (result, guess) in zip(result_history.iter(), guess_history.iter()) {
            writeln!(term, "{} {}", guess, result)?;
        }
        writeln!(term, "{}", message_to_user)?;
        term.clear_line()?;
        write!(term, "> ")?;

        if won {
            break;
        }

        // read in guess
        let guess = term.read_line()?.trim().to_ascii_lowercase();

        // check and set errors
        if guess.len() != 5 {
            message_to_user = "guess must be 5 characters".to_owned();
        } else if !word_list.contains(&guess) {
            message_to_user = format!("\"{}\" is not a word", guess);
        } else {
            let guess_arr: [char; 5] = guess
            .chars().into_iter()
            .collect::<Vec<char>>().try_into().unwrap();

            // compute guess
            let result = game.guess(guess_arr);

            // convert result into something renderable
            let output = result.iter().map(|class_type| match class_type {
                wordle::MatchClass::NoMatch => "⬛".to_owned(),
                wordle::MatchClass::PartialMatch => "🟨".to_owned(),
                wordle::MatchClass::ExactMatch => "🟩".to_owned(),
            }).collect::<Vec<String>>().join("");

            // set up next render loop
            result_history[guess_idx] = output;
            guess_history[guess_idx] = guess;
            guess_idx += 1;

            if result.iter().all(|match_class| *match_class == wordle::MatchClass::ExactMatch) {
                message_to_user = format!("you win in {} guesses", guess_idx).to_owned();
                won = true;
            }
        }

        // reset for next render
        term.move_cursor_up(8)?;
    }

    if !won {
        writeln!(term, "you failed, the word was \"{}\"", answer_str)?;
    }

    Ok(())
}
