mod typing_test;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Attribute, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use std::{
    io::{self, stdout, Stdout},
};
use typing_test::TypingTest;

enum UserInput {
    Char(char),
    Backspace,
    Tab,
    Kill,
}

struct WordCursor {
    word_chars: Vec<char>,
    cur: usize,
}

impl<'a> WordCursor {
    fn new(word: &'a String) -> Self {
        let word_chars = word.chars().collect();
        Self { word_chars, cur: 0 }
    }

    fn peek(&mut self) -> Option<&char> {
        self.word_chars.get(self.cur)
    }

    fn next(&mut self) -> Option<&char> {
        let char = self.word_chars.get(self.cur + 1);
        self.cur += 1;
        char
    }

    fn prev(&mut self) -> Option<&char> {
        let prev_cur = if self.cur == 0 { 0 } else { self.cur - 1 };
        let char = self.word_chars.get(prev_cur);
        self.cur = prev_cur;
        char
    }

    fn push(&mut self, c: char) {
        self.word_chars.push(c)
    }

    fn word(&self) -> String {
        self.word_chars.iter().collect()
    }
}

struct Renderer {
    stdout: Stdout,
}

impl Renderer {
    fn init(&mut self) -> Result<(), std::io::Error> {
        execute!(self.stdout, EnterAlternateScreen)
    }

    fn leave(&mut self) -> Result<(), std::io::Error> {
        execute!(self.stdout, LeaveAlternateScreen)
    }

    fn render(&mut self, text: String) {
        queue!(self.stdout, Clear(ClearType::All)).unwrap();
        execute!(
            self.stdout,
            // Blue foreground
            SetForegroundColor(Color::Blue),
            // Red background
            SetBackgroundColor(Color::Red),
            // Print text
            Print(text),
            // Reset to default colors
            ResetColor
        )
        .unwrap();
    }
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
    let stdout = stdout();

    let mut renderer = Renderer {
        stdout
    };

    renderer.init()?;

    loop {
        let next_word = match typing_test.next_word() {
            Some(w) => w,
            None => break,
        };

        let tmp_word = String::from("");
        let mut target_word = WordCursor::new(&next_word);
        let mut user_word = WordCursor::new(&tmp_word);

        loop {

            // TUI
            renderer.render(format!("{}\n{}\n", next_word, user_word.word()));

            let expected_char = match target_word.peek() {
                Some(&c) => c,
                None => break,
            };

            match user_input().expect("should have read user input") {
                UserInput::Char(user_char) => {
                    user_word.push(user_char);
                    if user_char != expected_char || target_word.cur != user_word.cur {
                        println!("Wrong. Try again.")
                    } else {
                        target_word.next();
                    }
                    user_word.next();
                }

                UserInput::Backspace => {
                    user_word.prev();
                    user_word.word_chars.pop();
                }

                UserInput::Tab => {
                    todo!("need to implement Tab");
                }

                UserInput::Kill => return Ok(()),
            }
        }
        println!("You wrote the complete word!");
    }
    renderer.leave()
}
