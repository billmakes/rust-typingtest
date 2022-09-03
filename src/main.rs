use colored::{ColoredString, Colorize};
use std::io;
use std::time::Instant;

const WINDOW_LEN: usize = 4;

fn main() {
    let word_list = type_t::create_wordlist();

    let mut score = Score::new();
    let timer = Instant::now();
    loop {
        if timer.elapsed().as_secs() >= 60 {
            break;
        }
        for curr_word in word_list.windows(WINDOW_LEN) {
            let mut word_input = String::new();
            // this clears the screen
            print!("{}[2J", 27 as char);
            score.print();
            for (idx, item) in curr_word.iter().enumerate() {
                let mut item_str: ColoredString = item.purple();

                if idx == 0 {
                    item_str = item.reversed();
                }

                if idx == curr_word.len() - 1 {
                    println!("{item_str}");
                } else {
                    print!("{item_str} ");
                }
            }

            io::stdin()
                .read_line(&mut word_input)
                .expect("failed to read line");

            if curr_word[0] == word_input.trim() {
                score.correct(curr_word[0].len() as u64);
            } else {
                score.wrong();
            }
        }
    }

    print!("{}[2J", 27 as char);
    println!("YOU'RE DONE!!");
    score.print();
    println!("WPM: {}", score.calcuate_wpm());
    // debug log
    println!("time elapsed {}", timer.elapsed().as_secs());
}

struct Score {
    correct: usize,
    wrong: usize,
    chars_typed: u64,
}

impl Score {
    fn new() -> Self {
        Self {
            correct: 0,
            wrong: 0,
            chars_typed: 0,
        }
    }

    fn calcuate_wpm(&self) -> u64 {
        self.chars_typed as u64 / 5
    }

    fn print(&self) {
        println!("characters typed: {}", self.chars_typed);
        let correct = format!("correct: {}", self.correct).green();
        println!("{}", correct);
        let wrong = format!("wrong: {}", self.wrong).red();
        println!("{}", wrong);
    }

    fn correct(&mut self, chars: u64) {
        self.correct += 1;
        self.chars_typed += chars;
    }

    fn wrong(&mut self) {
        self.wrong += 1;
    }
}
