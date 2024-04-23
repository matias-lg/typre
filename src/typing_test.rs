use std::fs;

pub struct TypingTest {
    word_list: Vec<String>,
    current_word: usize,
}

const MAX_WORDS: usize = 20;

impl TypingTest {
    pub fn from_file(file_path: &str) -> Self {

        let word_list: Vec<String> = fs::read_to_string(file_path)
            .expect("Should have been able to read the file")
            .split('\n')
            // don't consider words of length 1
            .filter(|w| w.len() > 1)
            .map(|s| String::from(s))
            .take(MAX_WORDS)
            .collect();

        Self {
            word_list,
            current_word: 0
        }
    }

    pub fn next_word(&mut self) -> Option<&String> {
        let tmp_word = self.word_list.get(self.current_word);
        self.current_word += 1;
        tmp_word
    }
}