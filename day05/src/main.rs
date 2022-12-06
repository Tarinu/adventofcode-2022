use common::{read_input, read_lines};

struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

impl From<String> for Move {
    fn from(line: String) -> Self {
        let split = line.split_whitespace().collect::<Vec<_>>();

        Self {
            amount: split.get(1).unwrap().parse().unwrap(),
            from: split.get(3).unwrap().parse::<usize>().unwrap() - 1,
            to: split.get(5).unwrap().parse::<usize>().unwrap() - 1,
        }
    }
}

fn initialize_stacks(input: &mut impl Iterator<Item = String>) -> Vec<Vec<char>> {
    let mut stack_lines = input
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let header = stack_lines.pop().unwrap();
    let stack_count = split_to_chunks(&header).iter().count();

    let mut stacks = Vec::with_capacity(stack_count);
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    stack_lines.reverse();

    for line in stack_lines {
        let chunks = split_to_chunks(&line);
        chunks.iter().enumerate().for_each(|(i, char)| {
            stacks
                .get_mut(i)
                .unwrap_or_else(|| panic!("No stack on index {}", i))
                .push(char.to_owned())
        });
    }

    for stack in stacks.iter_mut() {
        stack.retain(|char| !char.is_whitespace());
    }

    stacks
}

fn split_to_chunks(input: &String) -> Vec<char> {
    input
        .char_indices()
        .filter(|(i, _char)| i % 4 == 1)
        .map(|(_, char)| char)
        .collect::<Vec<_>>()
}

fn main() {
    let reader = read_input(env!("CARGO_CRATE_NAME"));
    let mut lines = read_lines(reader);

    let mut stacks = initialize_stacks(&mut lines);

    for line in lines {
        let change: Move = line.into();

        for _ in 0..change.amount {
            let value = stacks.get_mut(change.from).unwrap().pop().unwrap();
            stacks.get_mut(change.to).unwrap().push(value);
        }
    }

    let top = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    println!("Top elements: {}", top);
}
