use common::io::stdin;
use day11::Grid;

const PART_1_STEPS: usize = 100;

const DELTAS: [(i8, i8); 8] = [
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    let values = stdin::collect_lines_into_vec::<u8>("");

    let mut grid: Grid = values.try_into().unwrap();

    let mut part_1_solved = false;
    let mut part_2_solved = false;

    let mut total_flashes = 0;

    let mut step = 0;
    loop {
        part_1_solved |= step == PART_1_STEPS;
        part_2_solved |= grid == 0
            || grid == 1
            || grid == 2
            || grid == 3
            || grid == 4
            || grid == 5
            || grid == 6
            || grid == 7
            || grid == 8
            || grid == 9;

        if part_1_solved && part_2_solved {
            break;
        }

        grid += 1;

        let mut mask = Grid::from(!0);

        let mut flashing = mask & (grid / 10);

        while flashing != 0 {
            grid %= 10;
            mask ^= flashing * !0;

            if !part_1_solved {
                total_flashes += flashing.sum_values();
            }

            let flashing_neighbours = DELTAS
                .iter()
                .fold(Grid::from(0), |acc, delta| acc + flashing.translate(*delta));

            grid += mask & flashing_neighbours;

            let clamp = grid / 10;
            let not_clamp = (clamp ^ 1) * !0;

            grid = (not_clamp & grid) | (clamp * 10);

            flashing = mask & (grid / 10);
        }

        step += 1;
    }

    println!("{}", total_flashes);
    println!("{}", step);
}
