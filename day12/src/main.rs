use std::{
    collections::{HashMap, HashSet},
};

use common::io::stdin;

type Vertex = String;
type Graph = HashMap<Vertex, Vec<Vertex>>;
type Path = Vec<Vertex>;

fn construct_candidates(path: &Path, graph: &Graph) -> Vec<Vertex> {
    let mut in_sol = HashSet::new(); // [false; 1 << 8];
    for s in path {
        if !(*s == s.to_uppercase()) {
            in_sol.insert(s);
        }
    }

    if path.is_empty() {
        return vec!["start".to_string()];
    }

    let mut candidates = vec![];
    if let Some(last) = path.last() {
        if *last == "end".to_string() {
            return candidates;
        }
        if let Some(ns) = graph.get(last) {
            for v in ns {
                if !in_sol.contains(v) {
                    candidates.push(v.clone());
                }
            }
        }
    }
    candidates
}

fn backtrack(path: &mut Path, graph: &Graph, solution_count: &mut usize) {
    if path.last() == Some(&"end".to_string()) {
        *solution_count += 1;
        return;
    }
    let candidates = construct_candidates(path, graph);

    for candidate in candidates {
        path.push(candidate);
        backtrack(path, graph, solution_count);
        path.pop();
    }
}

fn main() {
    let edges = stdin::collect_into_vec_with(|line| {
        let mut tokens = line.trim().split('-');
        match (tokens.next(), tokens.next()) {
            (Some(u), Some(v)) => Some((u.to_string(), v.to_string())),
            _ => None,
        }
    });

    let mut graph = Graph::new();
    for (u, v) in edges {
        let neighbours_u = graph.entry(u.clone()).or_insert(Vec::new());
        neighbours_u.push(v.clone());
        let neighbours_v = graph.entry(v.clone()).or_insert(Vec::new());
        neighbours_v.push(u.clone());
    }

    let mut path = vec![];
    let mut solution_count = 0;

    backtrack(&mut path, &graph, &mut solution_count);

    println!("{}", solution_count);
}
