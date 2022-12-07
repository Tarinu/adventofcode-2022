use std::{collections::HashSet, io::BufRead};

use common::read_input;

fn find_start(input: &String, length: usize) -> usize {
    let mut end = length;

    while end <= input.len() {
        let slice = &input[end - length..end];
        let set: HashSet<char> = HashSet::from_iter(slice.chars());
        if set.len() == length {
            break;
        }

        end += 1;
    }

    end
}

fn main() {
    let mut reader = read_input(env!("CARGO_CRATE_NAME"));
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let packet = find_start(&line, 4);

    println!("Packet starts at: {}", packet);

    let message = find_start(&line, 14);

    println!("Message starts at: {}", message);
}
