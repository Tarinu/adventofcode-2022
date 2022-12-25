use std::collections::HashSet;

use common::{read_input, read_lines};

struct Rope {
    head: (i16, i16),
    tail: (i16, i16),
    tail_positions: HashSet<(i16, i16)>,
}

impl Rope {
    fn new() -> Self {
        let mut rope = Self {
            head: (0, 0),
            tail: (0, 0),
            tail_positions: HashSet::new(),
        };

        rope.tail_positions.insert(rope.tail);

        rope
    }

    fn move_head(&mut self, m: &Move) {
        let mut steps = m.steps;

        let vector = match m.direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        while steps > 0 {
            self.head.0 += vector.0;
            self.head.1 += vector.1;
            self.move_tail();
            steps -= 1;
        }
    }

    fn move_tail(&mut self) {
        let diff = (self.head.0 - self.tail.0, self.head.1 - self.tail.1);
        if diff.0.abs() < 2 && diff.1.abs() < 2 {
            return;
        }

        if diff.0 != 0 {
            self.tail.0 += diff.0 / diff.0.abs();
        }

        if diff.1 != 0 {
            self.tail.1 += diff.1 / diff.1.abs();
        }

        self.tail_positions.insert(self.tail);
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            dir => panic!("Invalid direction: {}", dir),
        }
    }
}

struct Move {
    direction: Direction,
    steps: u8,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let mut split = value.split_whitespace();

        Self {
            direction: split.next().unwrap().into(),
            steps: split.next().unwrap().parse::<_>().unwrap(),
        }
    }
}

fn main() {
    let reader = read_input(env!("CARGO_CRATE_NAME"));
    let lines = read_lines(reader);

    let mut rope = Rope::new();

    for line in lines {
        let m = Move::from(line.as_str());
        rope.move_head(&m);
    }

    println!("Tail has been in {} positions", rope.tail_positions.len());
}
