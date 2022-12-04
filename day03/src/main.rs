use std::collections::HashSet;

use common::{read_input, read_lines};

fn main() {
    let reader = read_input(env!("CARGO_CRATE_NAME"));
    let lines = read_lines(reader);

    let mut sum = 0;

    for line in lines {
        let half_point = line.len() / 2;
        let first_half = &line[..half_point];
        let second_half = &line[half_point..];

        let first_characters: HashSet<char> = HashSet::from_iter(first_half.chars());
        let second_characters: HashSet<char> = HashSet::from_iter(second_half.chars());

        let common_characters = first_characters.intersection(&second_characters);

        sum += common_characters
            .map(|char| {
                let point = *char as u32;
                if point <= 90 {
                    point - 65 + 27
                } else {
                    point - 97 + 1
                }
            })
            .sum::<u32>()
    }

    println!("Sum: {}", sum);
}
