use std::{
    io::{BufRead, Read},
    str::FromStr,
};

pub fn collect_into_vec<T: FromStr>() -> Vec<T> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok().map(|line| line.parse::<T>()))
        .filter_map(|value| value.ok())
        .collect()
}

pub fn collect_into_vec_with<T>(f: fn(&str) -> Option<T>) -> Vec<T> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok().map(|line| f(&line)).flatten())
        .collect()
}

pub fn read_line_with<T>(f: fn(&str) -> Option<T>) -> Option<T> {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    if let Ok(_) = stdin.read_line(&mut buf) {
        return f(&buf);
    }

    None
}

pub fn collect_lines_into_vec<T: FromStr>(delimiter: &str) -> Vec<T> {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();

    let mut result = Vec::new();
    if let Ok(_) = stdin.read_to_string(&mut buf) {
        result.extend(
            buf.trim()
                .split(delimiter)
                .filter_map(|lines| lines.parse::<T>().ok()),
        );
    }

    result
}
