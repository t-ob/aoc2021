use std::str::FromStr;

use common::io::collect_stdin;

enum Command {
    Up(u64),
    Down(u64),
    Forward(u64),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();
        let direction = tokens.next();
        let amount = tokens.next().map(|s| s.parse::<u64>().unwrap());
        match (direction, amount) {
            (Some("up"), Some(v)) => Ok(Command::Up(v)),
            (Some("down"), Some(v)) => Ok(Command::Down(v)),
            (Some("forward"), Some(v)) => Ok(Command::Forward(v)),
            _ => Err(()),
        }
    }
}

fn main() {
    let commands = collect_stdin::<Command>();
    
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal_position = 0;

    // Part 1
    for command in &commands {
        match command {
            Command::Up(v) => depth -= v,
            Command::Down(v) => depth += v,
            Command::Forward(v) => horizontal_position += v,
        }
    }
    println!("{}", depth * horizontal_position);

    // Part 2
    depth = 0;
    horizontal_position = 0;

    for command in &commands {
        match command {
            Command::Up(v) => aim -= v,
            Command::Down(v) => aim += v,
            Command::Forward(v) => {
                horizontal_position += v;
                depth += aim * v;
            }
        }
    }
    println!("{}", depth * horizontal_position);
}
