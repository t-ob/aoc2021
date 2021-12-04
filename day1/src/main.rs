use common::io::stdin::collect_into_vec;

fn main() {
    let values = collect_into_vec::<i64>();

    // Part 1
    let count_part_1 = values
        .iter()
        .zip(values.iter().skip(1))
        .filter(|(lhs, rhs)| lhs < rhs)
        .count();
    println!("{}", count_part_1);

    // Part 2
    let count_part_2 = values
        .iter()
        .zip(values.iter().skip(3)) // a + b + c < b + c + d iff a < d
        .filter(|(lhs, rhs)| lhs < rhs)
        .count();
    println!("{}", count_part_2);
}
