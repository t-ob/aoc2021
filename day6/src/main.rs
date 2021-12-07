use common::math::linalg;
use common::io::stdin;

static PART_1_DAYS: usize = 80;
static PART_2_DAYS: usize = 256;

fn advance(data: &mut [usize; 9]) {
    let next_zero = data[6] + data[8];
    for i in (1..9).rev() {
        data[i] = data[i - 1];
    }
    data[0] = next_zero;
}

fn main() {
    let initial_state = match stdin::read_csv_line::<usize>() {
        Some(initial_state) => initial_state,
        _ => return,
    };

    let mut counts = [0; 9];
    for days_remaining in initial_state {
        counts[days_remaining] += 1;
    }

    let mut dp = [1; 9];

    let mut iteration = 0;

    while iteration < PART_1_DAYS {
        advance( &mut dp);
        iteration += 1;
    }
    println!("{}", linalg::scalar_product(&counts, &dp));

    while iteration < PART_2_DAYS {
        advance(&mut dp);
        iteration += 1;
    }
    println!("{}", linalg::scalar_product(&counts, &dp));
}
