use std::{collections::HashSet, fs::File, io::BufReader, ptr::eq};

use common::{read_input, read_lines};

#[derive(Hash, Eq)]
struct Tree {
    height: u8,
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        eq(self, other)
    }
}

impl Tree {
    fn new(height: u8) -> Self {
        Self { height }
    }
}

struct Grid {
    grid: Vec<Vec<Tree>>,
}

impl From<BufReader<File>> for Grid {
    fn from(reader: BufReader<File>) -> Self {
        let lines = read_lines(reader);

        let mut grid = Vec::new();

        for line in lines {
            let grid_line = line
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .map(|digit| Tree::new(digit.try_into().unwrap()))
                .collect::<Vec<_>>();

            grid.push(grid_line);
        }

        Self { grid }
    }
}

impl Grid {
    fn get_visible_trees(&self) -> HashSet<&Tree> {
        let mut visible = HashSet::new();

        for (i, line) in self.grid.iter().enumerate() {
            for (j, tree) in line.iter().enumerate() {
                if !line[..j].iter().any(|prev| prev.height >= tree.height) {
                    visible.insert(tree);
                    continue;
                }

                if !line[j + 1..].iter().any(|next| next.height >= tree.height) {
                    visible.insert(tree);
                    continue;
                }

                if !self.grid[..i]
                    .iter()
                    .map(|line| &line[j])
                    .any(|t| t.height >= tree.height)
                {
                    visible.insert(tree);
                    continue;
                }

                if !self.grid[i + 1..]
                    .iter()
                    .map(|line| &line[j])
                    .any(|t| t.height >= tree.height)
                {
                    visible.insert(tree);
                    continue;
                }
            }
        }

        visible
    }

    fn get_highest_score(&self) -> u32 {
        let mut highest = 0;

        for (i, line) in self.grid.iter().enumerate() {
            for (j, tree) in line.iter().enumerate() {
                let mut left = 0;
                for prev in line[..j].iter().rev() {
                    left += 1;
                    if prev.height >= tree.height {
                        break;
                    }
                }

                let mut right = 0;
                for next in line[j + 1..].iter() {
                    right += 1;
                    if next.height >= tree.height {
                        break;
                    }
                }

                let mut up = 0;
                for prev in self.grid[..i].iter().rev().map(|line| &line[j]) {
                    up += 1;
                    if prev.height >= tree.height {
                        break;
                    }
                }

                let mut down = 0;
                for next in self.grid[i + 1..].iter().map(|line| &line[j]) {
                    down += 1;
                    if next.height >= tree.height {
                        break;
                    }
                }

                let score = [left, right, up, down].iter().product();
                if score > highest {
                    highest = score;
                }
            }
        }

        highest
    }
}

fn main() {
    let reader = read_input(env!("CARGO_CRATE_NAME"));
    let grid = Grid::from(reader);

    println!("Visible: {}", grid.get_visible_trees().len());

    println!("Highest Score: {}", grid.get_highest_score());
}
