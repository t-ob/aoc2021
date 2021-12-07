use common::io::stdin;

const POSITIONS: usize = 1 << 11;

fn main() {
    let positions = match stdin::read_csv_line::<u16>() {
        Some(positions) => positions,
        _ => return
    };

    let mut fuel_costs_part_1: [u32; POSITIONS] = [0; POSITIONS];
    let mut fuel_costs_part_2: [u32; POSITIONS] = [0; POSITIONS];

    for initial_position in positions {
        for destination in 0..POSITIONS {
            let distance = (initial_position as i16 - destination as i16).abs() as u32;
            fuel_costs_part_1[destination] += distance;
            fuel_costs_part_2[destination] += (distance * (distance + 1)) / 2;
        }
    }

    println!("{}", fuel_costs_part_1.iter().min().unwrap());
    println!("{}", fuel_costs_part_2.iter().min().unwrap());
}
