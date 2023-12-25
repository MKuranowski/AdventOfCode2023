// Copyright (c) 2023 Mikołaj Kuranowski
// SPDX-License-Identifier: MIT

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::stdin;

type Node = [u8; 3];

#[derive(Default)]
struct Edges(HashSet<(Node, Node)>);

impl Edges {
    fn insert(&mut self, edge: (Node, Node)) {
        self.0.insert(normalize(edge));
    }

    fn remove(&mut self, edge: (Node, Node)) -> bool {
        self.0.remove(&normalize(edge))
    }

    fn nodes(&self) -> HashSet<Node> {
        self.0.iter().flat_map(|(a, b)| [a, b]).copied().collect()
    }

    fn as_map(&self) -> HashMap<Node, Vec<Node>> {
        let mut map: HashMap<Node, Vec<Node>> = HashMap::default();
        for &(a, b) in &self.0 {
            map.entry(a).or_default().push(b);
            map.entry(b).or_default().push(a);
        }
        map
    }
}

fn normalize(edge: (Node, Node)) -> (Node, Node) {
    if edge.0 > edge.1 {
        (edge.1, edge.0)
    } else {
        edge
    }
}

fn load_input() -> Edges {
    let mut edges = Edges::default();
    for line in stdin().lines() {
        let line = line.expect("failed to read from stdin");
        let (from, tos) = line.split_once(": ").unwrap();
        let from_n: Node = from.as_bytes().try_into().unwrap();
        for to in tos.split(' ') {
            let to_n: Node = to.as_bytes().try_into().unwrap();
            edges.insert((from_n, to_n));
        }
    }
    edges
}

#[derive(Debug, PartialEq, Eq)]
struct DijkstraQueueEntry {
    at: Node,
    dist: u32,
}

impl PartialOrd for DijkstraQueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // XXX: Rust's BinaryHeap is a max-heap, Dijkstra requires a min-heap - hence the reverse
        Some(self.dist.cmp(&other.dist).reverse())
    }
}

impl Ord for DijkstraQueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // XXX: Rust's BinaryHeap is a max-heap, Dijkstra requires a min-heap - hence the reverse
        self.dist.cmp(&other.dist).reverse()
    }
}

fn dijkstra_paths(edges: &HashMap<Node, Vec<Node>>, from: Node) -> HashMap<Node, Node> {
    let mut distances: HashMap<Node, u32> = HashMap::default();
    let mut previous: HashMap<Node, Node> = HashMap::default();
    let mut visited: HashSet<Node> = HashSet::default();
    let mut queue: BinaryHeap<DijkstraQueueEntry> = BinaryHeap::default();

    distances.insert(from, 0);
    queue.push(DijkstraQueueEntry { at: from, dist: 0 });

    while let Some(entry) = queue.pop() {
        // Skip if visited
        if visited.contains(&entry.at) {
            continue;
        }
        visited.insert(entry.at);

        for neighbor in &edges[&entry.at] {
            let known_dist = distances.get(neighbor).copied().unwrap_or(u32::MAX);
            let alt_dist = entry.dist + 1;
            if alt_dist < known_dist {
                distances.insert(*neighbor, alt_dist);
                previous.insert(*neighbor, entry.at);
                queue.push(DijkstraQueueEntry {
                    at: *neighbor,
                    dist: alt_dist,
                });
            }
        }
    }

    return previous;
}

struct PathTracer<'a> {
    previous: &'a HashMap<Node, Node>,
    at: Node,
}

impl<'a> Iterator for PathTracer<'a> {
    type Item = (Node, Node);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&prev) = self.previous.get(&self.at) {
            let edge = normalize((prev, self.at));
            self.at = prev;
            Some(edge)
        } else {
            None
        }
    }
}

fn count_edge_usage(
    previous: &HashMap<Node, Node>,
    nodes: &HashSet<Node>,
    usage: &mut HashMap<(Node, Node), u32>,
) {
    for &start in nodes {
        let p = PathTracer {
            previous,
            at: start,
        };
        for edge in p {
            *usage.entry(edge).or_default() += 1;
        }
    }
}

fn find_candidates_to_cut(edges: &Edges) -> Vec<(Node, Node)> {
    // I'm assuming the 3 edges to cut serve as "choke points" for paths from nodes from the first
    // group to the second. This requires that the 2 groups are more-or-less the same size, and
    // shortest paths use the 3 "choke point" edges more-or-less uniformly.

    let edges_map = edges.as_map();
    let nodes = edges.nodes();

    // Count the usages of each way. Since this is a heuristic solution, not all possible paths
    // need to be examined (although that doesn't take too much time). It looks like examining
    // ~15 000 paths is enough (there are around 1500 nodes,
    // so this requires 10 calls to Dijkstra's algorithm).
    let mut usages: HashMap<(Node, Node), u32> = HashMap::default();
    for (_, &node) in nodes.iter().enumerate().take_while(|(idx, _)| *idx < 10) {
        let previous = dijkstra_paths(&edges_map, node);
        count_edge_usage(&previous, &nodes, &mut usages);
    }

    // Find the most used edges
    let usages_sorted = {
        let mut v: Vec<_> = usages.iter().map(|x| (*x.0, *x.1)).collect();
        v.sort_by(|a, b| a.1.cmp(&b.1).reverse());
        v
    };

    // Keep only the 3 most used edges
    (&usages_sorted[..3])
        .iter()
        .map(|(edge, _)| edge)
        .copied()
        .collect()
}

fn count_groups(edges: &Edges) -> (usize, usize) {
    let nodes = edges.nodes();
    let edges_map = edges.as_map();
    let mut group: HashSet<Node> = HashSet::default();
    let mut q: Vec<Node> = vec![*nodes.iter().next().unwrap()]; // start at an arbitrary node

    while let Some(node) = q.pop() {
        if group.insert(node) {
            for neighbor in &edges_map[&node] {
                if !group.contains(neighbor) {
                    q.push(*neighbor);
                }
            }
        }
    }

    (group.len(), nodes.len() - group.len())
}

fn main() {
    let mut edges = load_input();

    // Find the edges to cut
    let to_cut = find_candidates_to_cut(&edges);

    // Cut them
    for edge in to_cut {
        edges.remove(edge);
    }

    // Count the groups
    let (a, b) = count_groups(&edges);
    eprintln!("{a}\t{b}");

    let result = a * b;
    println!("{result}");
}
