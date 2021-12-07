use common::io::stdin::read_csv_line;

fn main() {
    if let Some(positions) = read_csv_line::<u16>() {
        let mut fuel_costs_part_1: [u32; 1 << 11] = [0; 1 << 11];
        let mut fuel_costs_part_2: [u32; 1 << 11] = [0; 1 << 11];
    
        for initial_position in positions {
            for destination in 0..(1 << 11) {
                let distance = (initial_position as i16 - destination as i16).abs() as u32;
                fuel_costs_part_1[destination] += distance;
                fuel_costs_part_2[destination] += (distance * (distance + 1)) / 2;
            }
        }
    
        println!("{}", fuel_costs_part_1.iter().min().unwrap());
        println!("{}", fuel_costs_part_2.iter().min().unwrap());
    }
}
