use std::{collections::HashSet, io::BufRead};

use common::read_input;

fn main() {
    let mut reader = read_input(env!("CARGO_CRATE_NAME"));
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let mut end = 4;

    while end <= line.len() {
        let slice = &line[end - 4..end];
        let set: HashSet<char> = HashSet::from_iter(slice.chars());
        if set.len() == 4 {
            break;
        }

        end += 1;
    }

    println!("Packet starts at: {}", end);
}
