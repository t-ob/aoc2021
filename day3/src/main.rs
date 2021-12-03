use common::io::collect_stdin_with;

fn bit_weight(bit_position: usize, values: &[u16]) -> u16 {
    let mut ones = 0;
    let mut zeroes = 0;
    for value in values.iter() {
        ones += 1 & (value >> bit_position);
        zeroes += 1 & (!value >> bit_position);
    }

    if ones >= zeroes {
        1
    } else {
        0
    }
}

fn main() {
    let values = collect_stdin_with(|s| u16::from_str_radix(s.trim(), 2).ok());

    let next_power_of_two_bit_position = values
        .iter()
        .min_by_key(|x| x.leading_zeros())
        .map(|x| 16 - x.leading_zeros())
        .unwrap();
    let mask: u16 = (1 << next_power_of_two_bit_position) - 1;

    // Part 1
    let mut weights = [0; 16];

    for (bit_position, weight) in (0..16).zip(weights.iter_mut().rev()) {
        *weight = bit_weight(bit_position, &values);
    }

    let gamma = weights.iter().fold(0, |a, v| v + (a << 1));

    let epsilon = !gamma & mask;

    println!("{}", gamma as u32 * epsilon as u32);

    // Part 2
    let mut oxygen_generator_values = values.clone();
    let mut co2_scrubber_values = values.clone();

    for (diagnostic_values, xor) in [
        (&mut oxygen_generator_values, 0),
        (&mut co2_scrubber_values, 1),
    ] {
        for bit_position in (0..next_power_of_two_bit_position as usize).rev() {
            if diagnostic_values.len() == 1 {
                break;
            }
            let weight = bit_weight(bit_position, diagnostic_values);
            let new_diagnostic_values = diagnostic_values
                .iter()
                .filter(|v| 1 & (**v >> bit_position) == weight ^ xor)
                .map(|v| *v)
                .collect::<Vec<u16>>();
            *diagnostic_values = new_diagnostic_values;
        }
    }

    let oxygen_generator_rating = oxygen_generator_values[0];
    let co2_scrubber_rating = co2_scrubber_values[0];

    println!(
        "{}",
        oxygen_generator_rating as u32 * co2_scrubber_rating as u32
    );
}
