use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for Hand {
    fn from(input: &str) -> Self {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Unknown symbol {}", input),
        }
    }
}

impl Hand {
    fn get_winning_hand(hand: &Hand) -> Hand {
        match hand {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn get_losing_hand(hand: &Hand) -> Hand {
        match hand {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn get_drawing_hand(hand: &Hand) -> Hand {
        hand.clone()
    }
}

struct Turn {
    player: Hand,
    opponent: Hand,
}

impl From<&str> for Turn {
    fn from(input: &str) -> Self {
        let mut parts = input.split_whitespace();

        Self {
            opponent: parts.next().unwrap().into(),
            player: parts.next().unwrap().into(),
        }
    }
}

impl From<Turnv2> for Turn {
    fn from(turn: Turnv2) -> Self {
        Self {
            player: match turn.result {
                Result::Win => Hand::get_winning_hand(&turn.opponent),
                Result::Lose => Hand::get_losing_hand(&turn.opponent),
                Result::Draw => Hand::get_drawing_hand(&turn.opponent),
            },
            opponent: turn.opponent,
        }
    }
}

impl Turn {
    fn get_result(&self) -> Result {
        self.into()
    }

    fn get_score(&self) -> u32 {
        let result = self.get_result();
        self.player as u32 + result as u32
    }
}

enum Result {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl From<&str> for Result {
    fn from(result: &str) -> Self {
        match result {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Unknown symbol {}", result),
        }
    }
}

impl From<&Turn> for Result {
    fn from(turn: &Turn) -> Self {
        match turn.player {
            Hand::Paper => match turn.opponent {
                Hand::Paper => Result::Draw,
                Hand::Rock => Result::Win,
                Hand::Scissors => Result::Lose,
            },
            Hand::Rock => match turn.opponent {
                Hand::Paper => Result::Lose,
                Hand::Rock => Result::Draw,
                Hand::Scissors => Result::Win,
            },
            Hand::Scissors => match turn.opponent {
                Hand::Paper => Result::Win,
                Hand::Rock => Result::Lose,
                Hand::Scissors => Result::Draw,
            },
        }
    }
}

struct Turnv2 {
    opponent: Hand,
    result: Result,
}

impl From<&str> for Turnv2 {
    fn from(input: &str) -> Self {
        let mut parts = input.split_whitespace();

        Self {
            opponent: parts.next().unwrap().into(),
            result: parts.next().unwrap().into(),
        }
    }
}

fn main() {
    let mut project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_root.push("input.txt");
    let input = File::open(project_root).expect("Input file not found");
    let input = BufReader::new(input);

    let mut turns = Vec::new();
    let mut correct_turns = Vec::new();

    for line in input.lines().map(|l| l.unwrap()) {
        let turn: Turn = <String as AsRef<str>>::as_ref(&line).into();
        turns.push(turn);

        let correct_turn: Turnv2 = <String as AsRef<str>>::as_ref(&line).into();
        correct_turns.push(Turn::from(correct_turn));
    }

    let score: u32 = turns.iter().map(|turn| turn.get_score()).sum();

    println!("Initial score: {}", score);

    let correct_score: u32 = correct_turns.iter().map(|turn| turn.get_score()).sum();

    println!("Correct score: {}", correct_score);
}
