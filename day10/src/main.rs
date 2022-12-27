use common::{read_input, read_lines};

#[allow(non_camel_case_types)]
enum Instruction {
    noop,
    addx(i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut split = value.split_whitespace();
        match split.next().unwrap() {
            "noop" => Instruction::noop,
            "addx" => Instruction::addx(split.next().unwrap().parse::<_>().unwrap()),
            ins => panic!("Unknown instruction: {}", ins),
        }
    }
}

impl Instruction {
    fn get_duration(&self) -> u8 {
        match self {
            Instruction::noop => 1,
            Instruction::addx(_) => 2,
        }
    }
}

fn main() {
    let reader = read_input(env!("CARGO_CRATE_NAME"));
    let mut lines = read_lines(reader);

    let mut cycle = 0;
    let check_cycles = [20, 60, 100, 140, 180, 220];
    let mut cycle_duration = 0;
    let mut register = 1;
    let mut signal_strengths = Vec::with_capacity(check_cycles.len());
    let mut add = 0;

    loop {
        cycle += 1;

        if check_cycles.contains(&cycle) {
            signal_strengths.push(register * cycle);
        }

        if cycle_duration > 0 {
            cycle_duration -= 1;
            if cycle_duration == 0 {
                register += add;
            }
            continue;
        }

        let line = lines.next();
        if line.is_none() {
            break;
        }

        let instruction = Instruction::from(line.unwrap().as_str());

        match instruction {
            Instruction::noop => (),
            Instruction::addx(x) => add = x,
        };

        cycle_duration = instruction.get_duration();
        cycle_duration -= 1;
    }

    println!(
        "Signal strength sum: {}",
        signal_strengths.iter().sum::<i32>()
    );
}
