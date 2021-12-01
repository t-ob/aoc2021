use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let values = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok().map(|line| line.parse::<i64>()))
        .filter_map(|value| value.ok())
        .collect::<Vec<_>>();

    // Part 1
    let count_part_1 = values
        .iter()
        .zip(values.iter().skip(1))
        .filter(|(lhs, rhs)| lhs < rhs)
        .count();
    println!("{}", count_part_1);

    // Part 2
    let windowed_values = values
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i64>>();
    let count_part_2 = windowed_values
        .iter()
        .zip(windowed_values.iter().skip(1))
        .filter(|(lhs, rhs)| lhs < rhs)
        .count();
    println!("{}", count_part_2);
}
