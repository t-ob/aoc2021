use std::collections::BinaryHeap;

use common::io::stdin;

const OPENERS: [char; 4] = ['(', '[', '{', '<'];
const CLOSERS: [char; 4] = [')', ']', '}', '>'];

fn main() {
    let lines = stdin::collect_into_vec_with(|line| Some(line.trim().chars().collect::<Vec<_>>()));

    let mut syntax_error_score_map: [u32; 1 << 7] = [0; 1 << 7];
    syntax_error_score_map[')' as usize] = 3;
    syntax_error_score_map[']' as usize] = 57;
    syntax_error_score_map['}' as usize] = 1197;
    syntax_error_score_map['>' as usize] = 25137;

    let mut completion_score_map: [u64; 1 << 7] = [0; 1 << 7];
    completion_score_map['(' as usize] = 1;
    completion_score_map['[' as usize] = 2;
    completion_score_map['{' as usize] = 3;
    completion_score_map['<' as usize] = 4;

    let mut syntax_error_score = 0;
    let mut completion_scores = BinaryHeap::new();
    for line in lines {
        let mut corrupted = false;
        let mut stack = Vec::new();
        for char in line {
            if corrupted {
                break;
            }
            if OPENERS.contains(&char) {
                stack.push(char);
                continue;
            }
            if CLOSERS.contains(&char) {
                match (stack.last(), char) {
                    (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {
                        stack.pop();
                    }
                    (_, c) => {
                        corrupted = true;
                        syntax_error_score += syntax_error_score_map[c as usize];
                    }
                }
            }
        }

        if !corrupted {
            let completion_score = stack
                .iter()
                .rev()
                .fold(0, |acc, c| 5 * acc + completion_score_map[*c as usize]);
            completion_scores.push(completion_score);
        }
    }

    println!("{}", syntax_error_score);

    for _ in 0..(completion_scores.len() / 2) {
        completion_scores.pop();
    }

    println!("{}", completion_scores.pop().unwrap());
}
