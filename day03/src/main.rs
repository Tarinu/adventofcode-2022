use std::collections::{HashMap, HashSet};

use common::{read_input, read_lines};

fn char_to_priority(char: &char) -> u32 {
    let point = *char as u32;
    if point <= 90 {
        point - 65 + 27
    } else {
        point - 97 + 1
    }
}

fn main() {
    let reader = read_input(env!("CARGO_CRATE_NAME"));
    let lines = read_lines(reader);

    let mut sum = 0;

    let mut rucksack_sets: Vec<HashSet<char>> = Vec::new();

    for line in lines {
        rucksack_sets.push(HashSet::from_iter(line.chars()));

        let half_point = line.len() / 2;
        let first_half = &line[..half_point];
        let second_half = &line[half_point..];

        let first_characters: HashSet<char> = HashSet::from_iter(first_half.chars());
        let second_characters: HashSet<char> = HashSet::from_iter(second_half.chars());

        let common_characters = first_characters.intersection(&second_characters);

        sum += common_characters.map(char_to_priority).sum::<u32>()
    }

    println!("Sum: {}", sum);

    let grouped_score = rucksack_sets
        .chunks(3)
        .map(|chunk| {
            chunk
                .iter()
                .fold(HashMap::new(), |mut accum, set| {
                    set.iter().for_each(|char| {
                        match accum.get(char) {
                            Some(count) => accum.insert(char, count + 1),
                            None => accum.insert(char, 1),
                        };
                    });

                    accum
                })
                .iter()
                .filter(|&(_char, count)| *count == 3)
                .map(|(char, _count)| *char)
                .collect::<Vec<&char>>()
        })
        .flatten()
        .map(char_to_priority)
        .sum::<u32>();

    println!("Grouped Score: {}", grouped_score);
}
