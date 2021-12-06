use common::io::stdin::read_line_with;

static PART_1_DAYS: usize = 80;
static PART_2_DAYS: usize = 256;

fn main() {
    if let Some(initial_state) = read_line_with(|line| {
        let tokens = line
            .trim()
            .split(',')
            .map(|t| t.parse::<usize>().ok())
            .collect::<Option<Vec<_>>>();
        tokens
    }) {
        let mut counts: [usize; 9] = [0; 9];
        for days_remaining in initial_state {
            counts[days_remaining] += 1;
        }

        let mut dp = [1; 9];

        let mut day = 0;

        while day < PART_2_DAYS {
            let next_zero = dp[6] + dp[8];
            for i in (1..9).rev() {
                dp[i] = dp[i - 1];
            }
            dp[0] = next_zero;

            day += 1;

            if day == PART_1_DAYS || day == PART_2_DAYS {
                println!(
                    "{}",
                    counts
                        .iter()
                        .zip(dp.iter())
                        .fold(0, |acc, (count, descendents)| acc + count * descendents)
                );
            }
        }
    }
}
