use common::io::stdin::read_line_with;

static PART_1_DAYS: usize = 80;
static PART_2_DAYS: usize = 256;

fn advance(iterations: usize, data: &mut [usize; 9]) {
    let mut iteration = 0;

    while iteration < iterations {
        let next_zero = data[6] + data[8];
        for i in (1..9).rev() {
            data[i] = data[i - 1];
        }
        data[0] = next_zero;

        iteration += 1;
    }
}

fn scalar_product(lhs: &[usize; 9], rhs: &[usize; 9]) -> usize {
    lhs.iter()
        .zip(rhs.iter())
        .fold(0, |acc, (s, t)| acc + s * t)
}

fn main() {
    if let Some(initial_state) = read_line_with(|line| {
        let tokens = line
            .trim()
            .split(',')
            .map(|t| t.parse::<usize>().ok())
            .collect::<Option<Vec<_>>>();
        tokens
    }) {
        let mut counts = [0; 9];
        for days_remaining in initial_state {
            counts[days_remaining] += 1;
        }

        let mut dp = [1; 9];

        advance(PART_1_DAYS, &mut dp);
        println!("{}", scalar_product(&counts, &dp));

        advance(PART_2_DAYS - PART_1_DAYS, &mut dp);
        println!("{}", scalar_product(&counts, &dp));
    }
}
