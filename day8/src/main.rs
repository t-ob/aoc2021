use common::io::stdin;

const CHARS: [u8; 7] = [1, 2, 4, 8, 16, 32, 64];
const MASK: u8 = 0x7f;

fn parse_signal(signal: &str) -> u8 {
    signal
        .chars()
        .map(|c| CHARS[c as usize - 'a' as usize])
        .fold(0, |acc, x| acc | x)
}

fn main() {
    let inputs: Vec<([u8; 10], [u8; 4])> = stdin::collect_into_vec_with(|line| {
        let mut iter = line.trim().split(" | ");

        match (iter.next(), iter.next()) {
            (Some(observed_signals), Some(outputs)) => {
                let observed_signals: [u8; 10] = observed_signals
                    .split_ascii_whitespace()
                    .map(parse_signal)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                let outputs: [u8; 4] = outputs
                    .split_ascii_whitespace()
                    .map(parse_signal)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                Some((observed_signals, outputs))
            }
            _ => None,
        }
    });

    let original_signals: [u8; 10] = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .map(parse_signal);

    let mut part_one_mapping: [usize; 1 << 8] = [0; 1 << 8];
    let mut part_one_count = 0;

    let mut part_two_mapping: [usize; 1 << 8] = [0; 1 << 8];
    let mut part_two_sum = 0;

    for (observed_signals, outputs) in inputs {
        let mut solution: [u8; 7] = [0; 7];

        let mut signals_by_bit_counts = vec![vec![]; 8];

        for signal in observed_signals {
            signals_by_bit_counts[signal.count_ones() as usize].push(signal);
        }

        let not_fives = MASK
            & (!signals_by_bit_counts[5][0]
                | !signals_by_bit_counts[5][1]
                | !signals_by_bit_counts[5][2]);

        let not_sixes = MASK
            & (!signals_by_bit_counts[6][0]
                | !signals_by_bit_counts[6][1]
                | !signals_by_bit_counts[6][2]);

        solution[0] = signals_by_bit_counts[2][0] ^ signals_by_bit_counts[3][0];
        solution[6] =
            MASK & !(signals_by_bit_counts[3][0] | signals_by_bit_counts[4][0] | not_sixes);
        solution[4] = MASK & !(solution[0] | solution[6] | signals_by_bit_counts[4][0]);
        solution[3] = MASK & !(solution[0] | solution[6] | not_fives);
        solution[1] = signals_by_bit_counts[4][0] ^ signals_by_bit_counts[2][0] ^ solution[3];
        solution[2] = not_sixes ^ solution[3] ^ solution[4];
        solution[5] = signals_by_bit_counts[2][0] ^ solution[2];

        let mapped_values: [u8; 10] =
            original_signals.map(|value| (0..7).fold(0, |acc, b| acc | (solution[b] * (1 & (value >> b)))));

        part_one_mapping[mapped_values[1] as usize] = 1;
        part_one_mapping[mapped_values[4] as usize] = 1;
        part_one_mapping[mapped_values[7] as usize] = 1;
        part_one_mapping[mapped_values[8] as usize] = 1;

        part_two_mapping[mapped_values[0] as usize] = 0;
        part_two_mapping[mapped_values[1] as usize] = 1;
        part_two_mapping[mapped_values[2] as usize] = 2;
        part_two_mapping[mapped_values[3] as usize] = 3;
        part_two_mapping[mapped_values[4] as usize] = 4;
        part_two_mapping[mapped_values[5] as usize] = 5;
        part_two_mapping[mapped_values[6] as usize] = 6;
        part_two_mapping[mapped_values[7] as usize] = 7;
        part_two_mapping[mapped_values[8] as usize] = 8;
        part_two_mapping[mapped_values[9] as usize] = 9;

        part_one_count += outputs
            .iter()
            .fold(0, |acc, v| acc + part_one_mapping[*v as usize]);
        part_two_sum += outputs
            .iter()
            .fold(0, |acc, v| 10 * acc + part_two_mapping[*v as usize]);

        part_one_mapping[mapped_values[1] as usize] = 0;
        part_one_mapping[mapped_values[4] as usize] = 0;
        part_one_mapping[mapped_values[7] as usize] = 0;
        part_one_mapping[mapped_values[8] as usize] = 0;

        part_two_mapping[mapped_values[0] as usize] = 0;
        part_two_mapping[mapped_values[1] as usize] = 0;
        part_two_mapping[mapped_values[2] as usize] = 0;
        part_two_mapping[mapped_values[3] as usize] = 0;
        part_two_mapping[mapped_values[4] as usize] = 0;
        part_two_mapping[mapped_values[5] as usize] = 0;
        part_two_mapping[mapped_values[6] as usize] = 0;
        part_two_mapping[mapped_values[7] as usize] = 0;
        part_two_mapping[mapped_values[8] as usize] = 0;
        part_two_mapping[mapped_values[9] as usize] = 0;
    }

    println!("{}", part_one_count);
    println!("{}", part_two_sum);
}
