use std::{collections::{HashMap, HashSet}, str::FromStr};

use common::io::stdin;

type Vertex = String;
type Graph = HashMap<Vertex, Vec<Vertex>>;
type Path = Vec<Vertex>;

fn construct_candidates(path: &Path, graph: &Graph, candidates: &mut Vec<Vertex>, nc: &mut usize) {
    let mut in_sol = HashSet::new(); // [false; 1 << 8];
    // for i in 0..k {
    //     if let Some(s) = path[i].clone() {
    //         if !(s == s.to_uppercase()) {
    //             in_sol.insert(s);
    //         }
    //     }
    // }
    for s in path {
        if !(*s == s.to_uppercase()) {
            in_sol.insert(s);
        }
    }

    if path.is_empty() {
        candidates.push("start".to_string());
        *nc = 1;
        return;
    }

    *nc = 0;
    if let Some(last) = path.last() {
        if *last == "end".to_string() {
            return
        }
        if let Some(ns) = graph.get(last) {
            for v in ns {
                if !in_sol.contains(v) {
                    candidates.push(v.clone());
                    *nc += 1;
                }
            }
        }
    }
}

fn backtrack(path: &mut Path, graph: &Graph, solution_count: &mut usize) {
    let mut candidates = vec![];
    let mut nc = 0;
    if path.last() == Some(&"end".to_string()) {
        *solution_count += 1;
        return;
    }
    construct_candidates(path, graph, &mut candidates, &mut nc);

    for i in 0..nc {
        path.push(candidates[i].clone());
        backtrack(path,  graph, solution_count);
        path.pop();
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

    let mut path = vec![];
    let mut solution_count = 0;

    backtrack(&mut path,  &graph, &mut solution_count);
    
    println!("{}", solution_count);
}
