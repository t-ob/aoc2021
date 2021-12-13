use std::{collections::{HashMap, HashSet}, str::FromStr};

const START: u8 = 0;
const END: u8 = u8::MAX;

use common::io::stdin;

fn construct_candidates(path: &[u8; 1 << 8], k: usize, graph: &HashMap<u8, Vec<u8>>, candidates: &mut [u8; 1 << 8], nc: &mut usize) {
    let mut in_sol = [false; 1 << 8];
    for i in 0..k {
        let c = path[i] as char;
        in_sol[c as usize] = !(c.is_ascii_alphabetic() && c.is_uppercase());
    }

    if k == 0 {
        candidates[0] = START;
        *nc = 1;
        return;
    }

    *nc = 0;
    let last = path[k-1];
    if last == END {
        return;
    }
    if let Some(ns) = graph.get(&last) {
        for v in ns {
            if !in_sol[*v as usize] {
                candidates[*nc] = *v;
                *nc += 1;
            }
        }
    }

    //print_path(path, k);
}

fn backtrack(path: &mut [u8; 1 << 8], k: usize, graph: &HashMap<u8, Vec<u8>>, solution_count: &mut usize) {
    let mut candidates = [0; 1 << 8];
    let mut nc = 0;
    if path[k-1] == END {
        print_path(&path, k);

        *solution_count += 1;
        return;
    }
    construct_candidates(path, k, graph, &mut candidates, &mut nc);

    for i in 0..nc {
        path[k] = candidates[i];
        backtrack(path, k + 1, graph, solution_count);
    }
}

// fn vertex_as_u8(s: &str) -> u8 {
//     match s {
//         "start" => 0,
//         "end" => u8::MAX,
//         _ => s.chars().fold(init, f)
//         _ => s.chars().next().unwrap() as u8
//     }
// }

fn label(u: u8) -> String {
    match u {
        0 => "start".to_string(),
        u8::MAX => "end".to_string(),
        b => format!("{}", (b as char)),
    }
}

fn print_path(path: &[u8; 1 << 8], k: usize) {
    let mut nodes = Vec::new();
    for i in 0..k {
        let n = label(path[i]);
        nodes.push(n);
        if path[i] == u8::MAX {
            break;
        }
    }

    println!("{}", nodes.join(" -> "));
}

fn main() {
    let edges = stdin::collect_into_vec_with(|line| {
        let mut tokens = line.trim().split('-');
        match (tokens.next(), tokens.next()) {
            (Some(u), Some(v)) => Some((vertex_as_u8(u), vertex_as_u8(v))),
            _ => None
        }});

    let mut graph = HashMap::new();
    for (u, v) in edges {
        let neighbours_u = graph.entry(u).or_insert(Vec::new());
        neighbours_u.push(v);
        let neighbours_v = graph.entry(v).or_insert(Vec::new());
        neighbours_v.push(u);
    }

    println!("{:?}", graph.iter().map(|(k, v)| (label(*k), v.iter().map(|u| label(*u)).collect::<Vec<_>>())).collect::<HashMap<_, _>>());

    let mut path = [0; 1 << 8];
    let mut solution_count = 0;

    backtrack(&mut path, 1, &graph, &mut solution_count);
    

    println!("{}", solution_count);
}
