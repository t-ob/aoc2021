use std::collections::VecDeque;

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
        .map(|x| 16 - x.leading_zeros() as usize)
        .max()
        .unwrap();

    // Part 1
    let mask: u16 = (1 << next_power_of_two_bit_position) - 1;
    let gamma = (0..next_power_of_two_bit_position)
        .rev()
        .fold(0, |acc, bit_position| {
            bit_weight(bit_position, &values) + (acc << 1)
        });
    let epsilon = !gamma & mask;

    println!("{}", gamma as u32 * epsilon as u32);

    // Part 2
    let mut oxygen_generator_values = VecDeque::from(values.clone());
    let mut co2_scrubber_values = VecDeque::from(values.clone());

    for (diagnostic_values, y) in [
        (&mut oxygen_generator_values, 0),
        (&mut co2_scrubber_values, 1),
    ] {
        for bit_position in (0..next_power_of_two_bit_position).rev() {
            if diagnostic_values.len() == 1 {
                break;
            }
            let next_diagnostic_values = diagnostic_values.drain(..).collect::<Vec<_>>();
            let weight = bit_weight(bit_position, &next_diagnostic_values);
            for value in next_diagnostic_values {
                if 1 & (value >> bit_position) == weight ^ y {
                    (*diagnostic_values).push_back(value);
                }
            }
        }
    }

    let oxygen_generator_rating = oxygen_generator_values.pop_front().unwrap();
    let co2_scrubber_rating = co2_scrubber_values.pop_front().unwrap();

    println!(
        "{}",
        oxygen_generator_rating as u32 * co2_scrubber_rating as u32
    );
}
