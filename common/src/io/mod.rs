use std::{io::BufRead, str::FromStr};

pub fn collect_stdin<T: FromStr>() -> Vec<T> {
    let stdin = std::io::stdin();
    let values = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok().map(|line| line.parse::<T>()))
        .filter_map(|value| value.ok())
        .collect::<_>();

    return values
}