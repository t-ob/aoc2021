use std::{io::BufRead, str::FromStr};

pub fn collect_stdin<T: FromStr>() -> Vec<T> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok().map(|line| line.parse::<T>()))
        .filter_map(|value| value.ok())
        .collect()
}

pub fn collect_stdin_with<T>(f: fn(&str) -> Option<T>) -> Vec<T> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok().map(|line| f(&line)).flatten())
        .collect()
}