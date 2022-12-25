use std::collections::HashSet;

use common::{read_input, read_lines};

struct Rope {
    head: (i16, i16),
    knots: Vec<(i16, i16)>,
    tail_positions: HashSet<(i16, i16)>,
}

impl Rope {
    fn new(length: u8) -> Self {
        let mut set = HashSet::new();
        set.insert((0, 0));

        let mut knots = Vec::with_capacity((length - 1).into());
        for _ in 0..length - 1 {
            knots.push((0, 0));
        }

        Self {
            head: (0, 0),
            knots,
            tail_positions: set,
        }
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

            let mut parent = &self.head;
            let length = self.knots.len();

            for (i, knot) in self.knots.iter_mut().enumerate() {
                Rope::pull_knots(&parent, knot);
                parent = knot;
                if i == length - 1 {
                    self.tail_positions.insert(*knot);
                }
            }

            steps -= 1;
        }
    }

    fn pull_knots(parent: &(i16, i16), knot: &mut (i16, i16)) {
        let diff = (parent.0 - knot.0, parent.1 - knot.1);
        if diff.0.abs() < 2 && diff.1.abs() < 2 {
            return;
        }

        if diff.0 != 0 {
            knot.0 += diff.0 / diff.0.abs();
        }

        if diff.1 != 0 {
            knot.1 += diff.1 / diff.1.abs();
        }
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

    let mut rope = Rope::new(2);
    let mut long_rope = Rope::new(10);

    for line in lines {
        let m = Move::from(line.as_str());
        rope.move_head(&m);
        long_rope.move_head(&m);
    }

    println!("Tail has been in {} positions", rope.tail_positions.len());
    println!(
        "Long rope's tail has been in {} positions",
        long_rope.tail_positions.len()
    );
}
