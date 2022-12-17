use std::collections::HashMap;
use std::hash::Hash;
use std::io::BufReader;
use std::{fmt::Display, fs};

use common::{read_input, read_lines};

#[derive(PartialEq)]
enum NodeType {
    Directory,
    File,
}

struct Node {
    name: String,
    size: u32,
    node_type: NodeType,
    children: HashMap<String, Node>,
}

impl Node {
    fn new(name: String, size: u32, node_type: NodeType) -> Self {
        Self {
            name,
            size,
            node_type,
            children: HashMap::new(),
        }
    }

    fn push(&mut self, path: &[String], name: String, size: u32, node_type: NodeType) {
        if path.len() == 0 {
            if !self.children.contains_key(&name) {
                self.children
                    .insert(name.clone(), Node::new(name, size, node_type));
            }
        } else {
            let dir = &path[0];
            if !self.children.contains_key(dir) {
                self.children.insert(
                    dir.clone(),
                    Node::new(dir.clone(), size, NodeType::Directory),
                );
            }

            let child = self.children.get_mut(dir).unwrap();

            child.push(&path[1..], name, size, node_type);
        }
    }

    fn print(&self, f: &mut std::fmt::Formatter<'_>, level: u8) -> std::fmt::Result {
        let line = match self.node_type {
            NodeType::Directory => format!("{} (dir)", self.name),
            NodeType::File => format!("{} (file size={})", self.name, self.size),
        };

        writeln!(f, "{}- {}", " ".repeat(level as usize * 2), line)?;

        for node in self.children.values() {
            node.print(f, level + 1)?;
        }

        Ok(())
    }

    fn get_size(&self) -> u32 {
        let mut sum = self.size;

        for child in self.children.values() {
            sum += child.get_size();
        }

        sum
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print(f, 0)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialEq<String> for Node {
    fn eq(&self, other: &String) -> bool {
        &self.name == other
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

struct Tree {
    root: Node,
}

impl Tree {
    fn new() -> Self {
        Self {
            root: Node::new("/".into(), 0, NodeType::Directory),
        }
    }

    fn push(&mut self, path: &Path, name: String, size: u32, node_type: NodeType) {
        self.root.push(&path.inner, name, size, node_type);
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}

struct Path {
    inner: Vec<String>,
}

impl Path {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn push(&mut self, path: String) {
        self.inner.push(path);
    }

    fn pop(&mut self) -> String {
        self.inner.pop().unwrap_or("/".to_string())
    }
}

fn parse_filesystem(reader: BufReader<fs::File>) -> Tree {
    let lines = read_lines(reader);
    let mut path = Path::new();
    let mut tree = Tree::new();

    for line in lines {
        let result = parse_line(line);
        match result {
            ParseResult::Command(command) => match command {
                Command::cd(dir) => match dir.as_str() {
                    "/" => continue,
                    ".." => {
                        path.pop();
                    }
                    _ => path.push(dir),
                },
                Command::ls => {}
            },
            ParseResult::File(file) => match file.file_type {
                FileType::Directory => {
                    tree.push(&path, file.name, 0, NodeType::Directory);
                }
                FileType::File(size) => {
                    tree.push(&path, file.name, size, NodeType::File);
                }
            },
        }
    }

    tree
}

#[allow(non_camel_case_types)]
enum Command {
    ls,
    cd(String),
}

struct File {
    file_type: FileType,
    name: String,
}

enum FileType {
    Directory,
    File(u32),
}

enum ParseResult {
    Command(Command),
    File(File),
}

fn parse_line(line: String) -> ParseResult {
    let prefix = line.chars().next().unwrap();
    if prefix == '$' {
        let mut split = line[2..].split_whitespace().into_iter();
        let command = split.next().unwrap();
        let command = if command == "ls" {
            Command::ls
        } else {
            Command::cd(split.next().unwrap().to_string())
        };

        ParseResult::Command(command)
    } else {
        let mut split = line.split_whitespace().into_iter();
        let size = split.next().unwrap();
        let name = split.next().unwrap();
        let file_type = if size == "dir" {
            FileType::Directory
        } else {
            FileType::File(size.parse().unwrap())
        };

        ParseResult::File(File {
            file_type,
            name: name.to_string(),
        })
    }
}

fn sum_dirs(root: &Node) -> u32 {
    let mut sum = 0;

    let size = root.get_size();

    if size <= 100_000 {
        sum += size;
    }

    for node in root.children.values() {
        if node.node_type == NodeType::File {
            continue;
        }

        sum += sum_dirs(node);
    }

    sum
}

fn filter_dirs(root: &Node) -> Vec<&Node> {
    let mut dirs = Vec::new();

    if root.node_type == NodeType::File {
        return dirs;
    }

    dirs.push(root);

    for node in root.children.values() {
        dirs.extend(filter_dirs(node));
    }

    dirs
}

fn main() {
    let reader = read_input(env!("CARGO_CRATE_NAME"));
    let tree = parse_filesystem(reader);

    println!("Sum of dirs under 100_000: {}", sum_dirs(&tree.root));

    let missing = 30_000_000 - (70_000_000 - tree.root.get_size());

    let mut dirs = filter_dirs(&tree.root)
        .iter()
        .map(|node| node.get_size())
        .collect::<Vec<u32>>();

    dirs.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    println!(
        "Dir size: {}",
        dirs.iter().find(|node| **node >= missing).unwrap()
    );
}
