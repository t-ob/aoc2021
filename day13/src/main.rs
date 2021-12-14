use common::io::stdin;

#[derive(Debug)]
enum Instruction {
    Left(usize),
    Up(usize),
}

const ROWS: usize = 1 << 11;
const COLS: usize = 1 << 11;

fn main() {
    let sections = stdin::collect_lines_into_vec::<String>("\n\n");

    let coordinates: Vec<(usize, usize)> = match sections.get(0) {
        Some(section) => {
            let mut coordinates = vec![];
            let lines = section.lines().map(|line| line.trim().split(','));
            for mut line in lines {
                if let (Some(Ok(x)), Some(Ok(y))) = (
                    line.next().map(|x| x.parse()),
                    line.next().map(|y| y.parse()),
                ) {
                    coordinates.push((x, y))
                }
            }
            coordinates
        }
        _ => return,
    };

    let instructions = match sections.get(1) {
        Some(section) => {
            let mut instructions = vec![];
            let lines = section.lines().map(|line| line.trim().split('='));
            for mut line in lines {
                match (line.next(), line.next().map(|x| x.parse())) {
                    (Some("fold along y"), Some(Ok(dy))) => instructions.push(Instruction::Up(dy)),
                    (Some("fold along x"), Some(Ok(dx))) => {
                        instructions.push(Instruction::Left(dx))
                    }
                    _ => {}
                }
            }
            instructions
        }
        _ => return,
    };

    let mut paper = vec![vec![0; COLS]; ROWS];

    let mut rows = ROWS;
    let mut cols = COLS;

    for (x, y) in coordinates {
        paper[y][x] = 1;
    }

    let mut part_1 = 0;

    for (iteration, instruction) in instructions.iter().enumerate() {
        let tmp = paper.clone();
        match instruction {
            Instruction::Up(dy) => {
                let to_write = paper.iter_mut().take(*dy).rev();
                let to_read = tmp.iter().skip(*dy + 1);

                for (row_write, row_read) in to_write.zip(to_read) {
                    for (x, y) in row_write
                        .iter_mut()
                        .take(cols)
                        .zip(row_read.iter().take(cols))
                    {
                        *x |= y;
                    }
                }

                rows = *dy;
            }
            Instruction::Left(dx) => {
                let to_write = paper.iter_mut().take(rows);
                let to_read = tmp.iter().take(rows);

                for (row_write, row_read) in to_write.zip(to_read) {
                    let vals_to_write = row_write.iter_mut().take(*dx).rev();
                    let vals_to_read = row_read.iter().skip(*dx + 1);
                    for (x, y) in vals_to_write.zip(vals_to_read) {
                        *x |= *y;
                    }
                }

                cols = *dx;
            }
        }

        if iteration == 0 {
            for row in paper.iter().take(rows) {
                for x in row.iter().take(cols) {
                    part_1 += *x as usize;
                }
            }
        }
    }

    println!("{}", part_1);

    // Part 2 code
    for row in paper.iter().take(rows) {
        for x in row.iter().take(cols) {
            match x {
                0 => print!("."),
                1 => print!("#"),
                _ => {}
            }
        }
        println!();
    }
}
