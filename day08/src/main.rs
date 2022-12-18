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
}

fn main() {
    let reader = read_input(env!("CARGO_CRATE_NAME"));
    let grid = Grid::from(reader);

    println!("Visible: {}", grid.get_visible_trees().len());
}
