use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn main() {
    let mut project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_root.push("input.txt");
    let input = File::open(project_root).expect("Input file not found");
    let input = BufReader::new(input);

    let mut elves = Vec::new();
    let mut total_calories = 0;

    for line in input.lines() {
        let value = line.unwrap();
        if value.is_empty() {
            elves.push(total_calories);
            total_calories = 0;
            continue;
        }

        total_calories += value.parse::<u32>().unwrap();
    }

    elves.push(total_calories);

    elves.sort_unstable();
    elves.reverse();

    println!("Max: {}", elves.first().unwrap());

    println!("{:?}", &elves[..3]);

    let top3 = elves[..3]
        .iter()
        .fold(0, |accum, calories| accum + calories);

    println!("Top 3 combined: {}", top3);
}
