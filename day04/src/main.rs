use std::collections::HashSet;

use common::{read_input, read_lines};

struct Section {
    set: HashSet<u8>,
}

impl From<&str> for Section {
    fn from(range: &str) -> Self {
        let mut split = range.split('-');

        let start = split.next().unwrap().parse::<u8>().unwrap();
        let end = split.next().unwrap().parse::<u8>().unwrap();

        Self {
            set: HashSet::from_iter(start..=end),
        }
    }
}

fn main() {
    let reader = read_input(env!("CARGO_CRATE_NAME"));
    let lines = read_lines(reader);

    let mut overlaps = 0;

    for line in lines {
        let mut sections = line.split(',');
        let first = Section::from(sections.next().unwrap());
        let second = Section::from(sections.next().unwrap());

        if first.set.is_subset(&second.set) || first.set.is_superset(&second.set) {
            overlaps += 1;
        }
    }

    println!("Overlaps: {}", overlaps);
}
