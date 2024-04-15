use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::seq::SliceRandom;

use std::io::{self, Error, Write};

use std::fs;

pub enum UserInput {
    Char(char),
    Backspace,
    Kill,
}

fn words_from_file(file_path: &str) -> Vec<String> {
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    content
        .split('\n')
        // don't consider words of length 1
        .filter(|w| w.len() > 1)
        .map(|s| String::from(s))
        .collect()
}

pub fn read_char() -> Result<UserInput, ()> {
    enable_raw_mode().expect("should have enabled raw mode");
    let e = match event::read() {
        Ok(Event::Key(key_event)) => key_event,
        _ => return Err(()),
    };
    disable_raw_mode().expect("should have disabled raw mode");

    match e {
        // Single char
        KeyEvent {
            modifiers: KeyModifiers::NONE,
            code: KeyCode::Char(c),
            ..
        } => Ok(UserInput::Char(c)),
        // CTRL + C
        KeyEvent {
            modifiers: KeyModifiers::CONTROL,
            code: KeyCode::Char('c'),
            ..
        } => Ok(UserInput::Kill),
        // Backspace
        KeyEvent {
            code: KeyCode::Backspace,
            ..
        } => Ok(UserInput::Backspace),
        _ => todo!("Inputs we don't handle yet"),
    }
}

fn main() -> io::Result<()> {
    let en_1000_words_path = "./src/words/top_1000_en.txt";
    let words = words_from_file(en_1000_words_path);

    loop {
        let next_word = words
            .choose(&mut rand::thread_rng())
            .expect("Should have been able to choose a random word");

        println!("{}", next_word);
        let mut word_iter = next_word.as_bytes().iter().peekable();

        // read user input char by char
        loop {
            let expected_char = match word_iter.peek() {
                Some(&&c) => c as char,
                None => break,
            };

            match read_char().expect("should have read user input") {
                UserInput::Char(user_char) => {
                    if user_char == expected_char {
                        word_iter.next();
                    } else {
                        println!("Wrong. Try again.")
                    }
                }
                UserInput::Backspace => {
                    todo!("need to implement Backspace");
                },
                UserInput::Kill => return Ok(()),
            }
        }
        println!("You wrote the complete word!");
    }
}
