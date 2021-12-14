use std::collections::{HashMap, HashSet};

use common::io::stdin;

type Vertex = u8;
type Graph = HashMap<Vertex, Vec<Vertex>>;

enum Policy {
    Part1,
    Part2,
}

fn construct_candidates(
    path: &[Vertex],
    graph: &Graph,
    small_rooms: &[u8; 1 << 8],
    start: &Vertex,
    end: &Vertex,
    rooms_visited: &[usize; 1 << 8],
    policy: &Policy,
) -> Vec<Vertex> {
    let last = match path.last() {
        Some(last) => last,
        _ => return vec![*start]
    };

    let mut invalid_candidates = [false; 1 << 8];

    let mut small_rooms_visited: [usize; 1 << 8] = [0; 1 << 8];
    for (small_room, (times_visited, room)) in small_rooms_visited.iter_mut().zip(rooms_visited.iter().zip(small_rooms.iter())) {
        *small_room = *times_visited * *room as usize;
    }

    match policy {
        Policy::Part1 => {
            for (room, times_visited) in (0usize..(1<<8)).zip(small_rooms_visited) {
                invalid_candidates[room] = times_visited > 0;
            }
        }
        Policy::Part2 => {
            invalid_candidates[*start as usize] = true;
            let small_visited_twice = small_rooms_visited.iter().any(|v| *v == 2);
            if small_visited_twice {
                for (room, times_visited) in (0usize..(1<<8)).zip(small_rooms_visited) {
                    invalid_candidates[room] = times_visited > 0;
                }
            }
        }
    }

    let mut candidates = vec![];
    if last == end {
        return candidates;
    }
    if let Some(neighbours) = graph.get(last) {
        for neighbouring_room in neighbours {
            if !invalid_candidates[*neighbouring_room as usize] {
                candidates.push(*neighbouring_room);
            }
        }
    }
    candidates
}

fn backtrack(
    graph: &Graph,
    small_rooms: &[u8; 1 << 8],
    start: &Vertex,
    end: &Vertex,
    policy: &Policy,
    path: &mut Vec<Vertex>,
    rooms_visited: &mut [usize; 1 << 8],
    solution_count: &mut usize,
) {
    if path.last() == Some(end) {
        *solution_count += 1;
        return;
    }
    let candidates = construct_candidates(path, graph, small_rooms, start, end, rooms_visited, policy);

    for candidate in candidates {
        path.push(candidate);
        rooms_visited[candidate as usize] = rooms_visited[candidate as usize] + 1;
        backtrack(graph, small_rooms, start, end, policy, path, rooms_visited, solution_count);
        rooms_visited[candidate as usize] = rooms_visited[candidate as usize] - 1;
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

    let mut small_rooms = [0; 1 << 8];

     for room in room_map
        .iter()
        .filter(|(k, _)| k.to_string() == k.to_lowercase())
        .map(|(_, v)| v) {
            small_rooms[*room as usize] = 1;
        }
    let start = room_map.get("start").unwrap();
    let end = room_map.get("end").unwrap();

    let mut graph = Graph::new();
    for (u, v) in &edges {
        let u = room_map.get(&u[..]).unwrap();
        let v = room_map.get(&v[..]).unwrap();
        let neighbours_u = graph.entry(*u).or_insert_with(Vec::new);
        neighbours_u.push(*v);
        let neighbours_v = graph.entry(*v).or_insert_with(Vec::new);
        neighbours_v.push(*u);
    }

    let mut path_part_1 = vec![];
    let mut rooms_visited_part_1 = [0; 1 << 8];
    let mut path_count_part_1 = 0;

    backtrack(
        &graph,
        &small_rooms,
        start,
        end,
        &Policy::Part1,
        &mut path_part_1,
        &mut rooms_visited_part_1,
        &mut path_count_part_1,
    );

    println!("{}", path_count_part_1);

    let mut path_part_2 = vec![];
    let mut rooms_visited_part_2 = [0; 1 << 8];
    let mut path_count_part_2 = 0;

    backtrack(
        &graph,
        &small_rooms,
        start,
        end,
        &Policy::Part2,
        &mut path_part_2,
        &mut rooms_visited_part_2,
        &mut path_count_part_2,
    );
    
    println!("{}", path_count_part_2);

}
