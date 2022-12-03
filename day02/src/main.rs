use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

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

struct Turn {
    player: Hand,
    opponent: Hand,
}

impl From<String> for Turn {
    fn from(input: String) -> Self {
        let mut parts = input.split_whitespace();

        Self {
            opponent: parts.next().unwrap().into(),
            player: parts.next().unwrap().into(),
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

fn main() {
    let mut project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_root.push("input.txt");
    let input = File::open(project_root).expect("Input file not found");
    let input = BufReader::new(input);

    let mut turns = Vec::new();

    for line in input.lines().map(|l| l.unwrap()) {
        let turn: Turn = line.into();
        turns.push(turn);
    }

    let score: u32 = turns.iter().map(|turn| turn.get_score()).sum();

    println!("{}", score);
}
