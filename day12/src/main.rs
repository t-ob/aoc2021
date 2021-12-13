use std::{collections::{HashMap, HashSet}, str::FromStr};

use common::io::stdin;

type Vertex = String;
type Graph = HashMap<Vertex, Vec<Vertex>>;
type Path = Vec<Option<Vertex>>;

fn construct_candidates(path: &Path, k: usize, graph: &Graph, candidates: &mut Vec<Option<Vertex>>, nc: &mut usize) {
    let mut in_sol = HashSet::new(); // [false; 1 << 8];
    for i in 0..k {
        if let Some(s) = path[i].clone() {
            if !(s == s.to_uppercase()) {
                in_sol.insert(s);
            }
        }
    }

    if k == 1 {
        candidates[0] = Some("start".to_string());
        *nc = 1;
        return;
    }

    *nc = 0;
    if let Some(last) = path[k-1].clone() {
        if last == "end".to_string() {
            return
        }
        if let Some(ns) = graph.get(&last) {
            for v in ns {
                if !in_sol.contains(v) {
                    candidates[*nc] = Some(v.clone());
                    *nc += 1;
                }
            }
        }
    }
}

fn backtrack(path: &mut Path, k: usize, graph: &Graph, solution_count: &mut usize) {
    let mut candidates = vec![None; 1 << 8];
    let mut nc = 0;
    if path[k-1] == Some("end".to_string()) {
        *solution_count += 1;
        return;
    }
    construct_candidates(path, k, graph, &mut candidates, &mut nc);

    for i in 0..nc {
        path[k] = candidates[i].clone();
        backtrack(path, k + 1, graph, solution_count);
    }
}

fn main() {
    let edges = stdin::collect_into_vec_with(|line| {
        let mut tokens = line.trim().split('-');
        match (tokens.next(), tokens.next()) {
            (Some(u), Some(v)) => Some((u.to_string(), v.to_string())),
            _ => None
        }});

    let mut graph = Graph::new();
    for (u, v) in edges {
        let neighbours_u = graph.entry(u.clone()).or_insert(Vec::new());
        neighbours_u.push(v.clone());
        let neighbours_v = graph.entry(v.clone()).or_insert(Vec::new());
        neighbours_v.push(u.clone());
    }

    let mut path = vec![None; 1 << 8];
    let mut solution_count = 0;

    backtrack(&mut path, 1, &graph, &mut solution_count);
    
    println!("{}", solution_count);
}
