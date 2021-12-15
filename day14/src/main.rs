use common::io::stdin;

fn digit(c: char) -> u16 {
    c as u16 - 'A' as u16 + 1
}

fn main() {
    let mut template = stdin::read_line_with::<Vec<u16>>(|line| Some(line.chars().map(digit).collect())).unwrap();

    let rules = stdin::collect_into_vec_with::<(u16, u16)>(|line| {
        let mut tokens = line.split(" -> ");
        match (tokens.next(), tokens.next()) {
            (Some(pair), Some(result)) => {
                let mut pair_chars = pair.chars();
                let mut result_chars = result.chars();
                match ((pair_chars.next(), pair_chars.next()), result_chars.next()) {
                    ((Some(c), Some(d)), Some(r)) => Some((27 * digit(c) + digit(d), digit(r))),
                    _ => return None
                }
            },
           _ => None
        }
    });

    let mut pairs = vec![];
    let mut mapping = [0; 1 << 10];
    for (pair, result) in rules {
        pairs.push(pair);
        mapping[pair as usize] = result;
    }

    let mut pair_counts = [0; 1 << 10];

    for (c, d) in template.iter().zip(template.iter().skip(1)) {
        pair_counts[27 * *c as usize + *d as usize] += 1;
    }

    let mut char_counts: [usize; 27] = [0; 27];
    for c in template.iter() {
        char_counts[*c as usize] += 1;
    }

    let mut iteration = 0;

    while iteration < 40 {
        let mut pair_diffs  = [0i64; 1 << 10];

        for pair in pairs.iter() {
            let pair_count = pair_counts[*pair as usize];
            if pair_count > 0 {
                let mapped_char = mapping[*pair as usize];
                let lhs = 27 * (pair / 27) + mapped_char;
                let rhs = 27 * mapped_char + pair % 27;
                pair_diffs[*pair as usize] -= pair_count;
                pair_diffs[lhs as usize] += pair_count;
                pair_diffs[rhs as usize] += pair_count;
                char_counts[mapped_char as usize] += pair_count as usize;
            }
        }

        for (x, y)in pair_counts.iter_mut().zip(pair_diffs) {
            *x += y;
        }

        iteration += 1;
    }

    println!("{:?}", char_counts);

    let most_frequent = char_counts.iter().max().unwrap();
    let least_frequent = char_counts.iter().filter(|c| **c > 0).min().unwrap();

    println!("{}", most_frequent - least_frequent);
}
