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
        let mut dp = [1; 18];

        let mut day = 0;

        while day < PART_2_DAYS {
            dp[0] = dp[15] + dp[17];
            for i in 1..9 {
                dp[i] = dp[i + 8];
            }
            for i in 9..18 {
                dp[i] = dp[i - 9];
            }

            day += 1;

            if day == PART_1_DAYS || day == PART_2_DAYS {
                println!(
                    "{}",
                    initial_state.iter().map(|idx| dp[*idx]).sum::<usize>()
                );
            }
        }
    }
}
