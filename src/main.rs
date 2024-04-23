mod typing_test;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use std::io::{self, stdout, Write};
use typing_test::TypingTest;

enum UserInput {
    Char(char),
    Backspace,
    Tab,
    Kill,
}

fn user_input() -> Result<UserInput, ()> {
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
    let mut typing_test = TypingTest::from_file(en_1000_words_path);
    let mut stdout = stdout();

    execute!(io::stdout(), EnterAlternateScreen)?;

    loop {
        let next_word = match typing_test.next_word() {
            Some(w) => w,
            None => break,
        };

        let word_chars: Vec<char> = next_word.chars().collect();
        let mut curr_char = 0;

        // read user input char by char
        let mut curr_user_char = 0;
        let mut user_word = String::from("");
        loop {
            queue!(stdout, Clear(ClearType::All))?;
            // print the TUI
            println!("{}", next_word);
            println!("{}", user_word);

            let expected_char = match word_chars.get(curr_char) {
                Some(&c) => c as char,
                None => break,
            };

            match user_input().expect("should have read user input") {
                UserInput::Char(user_char) => {
                    user_word.push_str(&String::from(user_char));
                    curr_user_char += 1;
                    if user_char != expected_char && curr_char != curr_user_char {
                        println!("Wrong. Try again.")
                    } else {
                        curr_char += 1;
                    }
                }
                UserInput::Backspace => {
                    curr_user_char = if curr_user_char == 0 { 0 } else { curr_user_char - 1 };
                    curr_char = if curr_char == 0 { 0 } else { curr_char - 1 };
                    user_word.pop();
                }

                UserInput::Tab => {
                    todo!("need to implement Tab");
                }

                UserInput::Kill => return Ok(()),
            }
        }
        println!("You wrote the complete word!");
    }
    execute!(io::stdout(), LeaveAlternateScreen)
}
