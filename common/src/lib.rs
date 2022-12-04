use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn read_input(project: &str) -> BufReader<File> {
    let mut project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project_root.pop();
    project_root.push(project);
    project_root.push("input.txt");
    let input = File::open(project_root).expect("Input file not found");
    BufReader::new(input)
}

pub fn read_lines(reader: BufReader<File>) -> impl Iterator<Item = String> {
    reader.lines().map(|l| l.unwrap())
}
