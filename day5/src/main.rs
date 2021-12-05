use std::cmp::max;

use common::geometry::{Point, Scalar};
use common::io::stdin::collect_into_vec_with;

fn main() {
    let line_segments = collect_into_vec_with(|line| {
        let mut points = line.split(" -> ").map(|token| token.parse::<Point<i32>>());
        match (points.next(), points.next()) {
            (Some(Ok(p)), Some(Ok(q))) => Some((p, q)),
            _ => None,
        }
    });

    let mut sea_bed_part_1 = vec![0; 1 << 20];
    let mut sea_bed_part_2 = vec![0; 1 << 20];

    for (mut p, q) in line_segments {
        let direction = q - p;
        let magnitude = Scalar::new(max(direction.x.abs(), direction.y.abs()));
        let normalised_direction = direction / magnitude;

        for _ in 0..(magnitude.v + 1) {
            let idx = (p.x << 10) as usize + (p.y & 0x3ff) as usize;

            // 0 => 0, 1 => 1, (x >= 2) => 3
            // right shifted later so that 0, 1 => 0, (x >= 2) => 1

            let count_part_1 = sea_bed_part_1[idx];
            sea_bed_part_1[idx] = 3
                & (count_part_1
                    | count_part_1 + (1 & (normalised_direction.x + normalised_direction.y)));

            let count_part_2 = sea_bed_part_2[idx];
            sea_bed_part_2[idx] = 3 & (count_part_2 | count_part_2 + 1);

            p += normalised_direction;
        }
    }

    // Part 1
    let count_part_1 = sea_bed_part_1.iter().map(|x| x >> 1).sum::<i32>();
    println!("{}", count_part_1);

    // Part 2
    let count_part_2 = sea_bed_part_2.iter().map(|x| x >> 1).sum::<i32>();
    println!("{}", count_part_2);
}
