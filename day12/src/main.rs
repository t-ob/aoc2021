use std::collections::{HashMap, HashSet};

use common::io::stdin;

type Vertex = u8;
type Graph = HashMap<Vertex, Vec<Vertex>>;
type Path = Vec<Vertex>;

enum Policy {
    Part1,
    Part2,
}

fn construct_candidates(
    path: &Path,
    graph: &Graph,
    small_rooms: &HashSet<Vertex>,
    start: &Vertex,
    end: &Vertex,
    policy: &Policy,
) -> Vec<Vertex> {
    if path.is_empty() {
        return vec![*start];
    }

    let mut invalid_candidates = HashSet::new();
    match policy {
        Policy::Part1 => {
            for vertex in path.iter().filter(|v| small_rooms.contains(*v)) {
                invalid_candidates.insert(vertex);
            }
        }
        Policy::Part2 => {
            invalid_candidates.insert(start);
            let small_rooms_visted = path.iter().filter(|v| small_rooms.contains(*v));
            let mut counts = HashMap::new();
            let mut small_visited_twice = false;
            for vertex in small_rooms_visted {
                let count = counts.entry(vertex).or_insert(0);
                *count += 1;
                small_visited_twice |= *count == 2;
            }
            if small_visited_twice {
                for vertex in counts.keys() {
                    invalid_candidates.insert(vertex);
                }
            }
        }
    }

    let mut candidates = vec![];
    if let Some(last) = path.last() {
        if last == end {
            return candidates;
        }
        if let Some(ns) = graph.get(last) {
            for v in ns {
                if !invalid_candidates.contains(v) {
                    candidates.push(v.clone());
                }
            }
        }
    }
    candidates
}

fn backtrack(
    graph: &Graph,
    small_rooms: &HashSet<Vertex>,
    start: &Vertex,
    end: &Vertex,
    policy: &Policy,
    path: &mut Path,
    solution_count: &mut usize,
) {
    if path.last() == Some(end) {
        *solution_count += 1;
        return;
    }
    let candidates = construct_candidates(path, graph, small_rooms, start, end, policy);

    for candidate in candidates {
        path.push(candidate);
        backtrack(graph, small_rooms, start, end, policy, path, solution_count);
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

    let mut rooms = HashSet::new();
    for (u, v) in &edges {
        rooms.insert(&u[..]);
        rooms.insert(&v[..]);
    }

    let room_map = rooms
        .iter()
        .enumerate()
        .map(|(idx, room)| (*room, idx as u8))
        .collect::<HashMap<_, _>>();

    let small_rooms = room_map
        .iter()
        .filter(|(k, v)| k.to_string() == k.to_lowercase())
        .map(|(_, v)| *v)
        .collect::<HashSet<_>>();
    let start = room_map.get("start").unwrap();
    let end = room_map.get("end").unwrap();

    let mut graph = Graph::new();
    for (u, v) in &edges {
        let u = room_map.get(&u[..]).unwrap();
        let v = room_map.get(&v[..]).unwrap();
        let neighbours_u = graph.entry(*u).or_insert(Vec::new());
        neighbours_u.push(*v);
        let neighbours_v = graph.entry(*v).or_insert(Vec::new());
        neighbours_v.push(*u);
    }

    let mut path_part_1 = vec![];
    let mut path_count_part_1 = 0;

    backtrack(
        &graph,
        &small_rooms,
        start,
        end,
        &Policy::Part1,
        &mut path_part_1,
        &mut path_count_part_1,
    );

    println!("{}", path_count_part_1);

    let mut path_part_2 = vec![];
    let mut path_count_part_2 = 0;

    backtrack(
        &graph,
        &small_rooms,
        start,
        end,
        &Policy::Part2,
        &mut path_part_2,
        &mut path_count_part_2,
    );
    
    println!("{}", path_count_part_2);

}
