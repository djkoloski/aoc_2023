use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

use common::{solve, Context, Input};

#[derive(Clone)]
struct Node {
    edges: Vec<usize>,
}

#[derive(Clone)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn add_node(&mut self) -> usize {
        let result = self.nodes.len();
        self.nodes.push(Node { edges: Vec::new() });
        result
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.nodes[from].edges.push(to);
        self.nodes[to].edges.push(from);
    }
}

impl Input for Graph {
    fn parse_reader<R: BufRead>(reader: R) -> common::Result<Self> {
        let mut labels = HashMap::new();
        let mut result = Graph::new();

        for line in reader.lines() {
            let line = line?;
            let (from, tos) =
                line.split_once(": ").context("expected node definition")?;
            let from = if let Some(node) = labels.get(from) {
                *node
            } else {
                let node = result.add_node();
                labels.insert(from.to_string(), node);
                node
            };

            for to in tos.split(' ') {
                let to = if let Some(node) = labels.get(to) {
                    *node
                } else {
                    let node = result.add_node();
                    labels.insert(to.to_string(), node);
                    node
                };

                result.add_edge(from, to);
            }
        }

        Ok(result)
    }
}

fn shortest_path(
    graph: &Graph,
    used: &HashSet<(usize, usize)>,
    from: usize,
    to: usize,
) -> Option<Vec<usize>> {
    let mut frontier = VecDeque::new();
    frontier.push_back((from, None));

    let mut visited = vec![false; graph.nodes.len()];
    let mut parents = vec![None; graph.nodes.len()];

    while let Some((node, parent)) = frontier.pop_front() {
        if visited[node] {
            continue;
        }
        visited[node] = true;
        parents[node] = parent;
        if node == to {
            break;
        }

        for edge in graph.nodes[node].edges.iter() {
            if !used.contains(&(node, *edge)) && !used.contains(&(*edge, node))
            {
                frontier.push_back((*edge, Some(node)));
            }
        }
    }

    let mut path = Vec::new();
    let mut current = to;
    while let Some(parent) = parents[current] {
        path.push(current);
        current = parent;
    }

    if path.is_empty() {
        None
    } else {
        path.push(from);
        path.reverse();
        Some(path)
    }
}

fn paths_between(
    graph: &Graph,
    used: &HashSet<(usize, usize)>,
    from: usize,
    to: usize,
) -> Option<usize> {
    let mut paths_found = 0;
    let mut used = used.clone();
    while let Some(path) = shortest_path(graph, &used, from, to) {
        paths_found += 1;
        for i in 0..path.len() - 1 {
            used.insert((path[i], path[i + 1]));
        }
        if paths_found > 3 {
            break;
        }
    }
    Some(paths_found)
}

fn find_sides(graph: &Graph) -> Option<(usize, usize)> {
    let from = 0;
    for to in 1..graph.nodes.len() {
        let paths = paths_between(graph, &HashSet::new(), from, to);
        if paths == Some(3) {
            return Some((from, to));
        }
    }

    None
}

fn count_nodes(
    graph: &Graph,
    used: &HashSet<(usize, usize)>,
    start: usize,
) -> usize {
    let mut visited = vec![false; graph.nodes.len()];
    let mut frontier = VecDeque::new();
    frontier.push_front(start);

    while let Some(node) = frontier.pop_front() {
        if visited[node] {
            continue;
        }

        visited[node] = true;
        for e in graph.nodes[node].edges.iter() {
            if !used.contains(&(node, *e)) && !used.contains(&(*e, node)) {
                frontier.push_back(*e);
            }
        }
    }

    visited.iter().filter(|x| **x).count()
}

fn main() -> common::Result<()> {
    solve(
        |input: &Graph| {
            let (a, b) = find_sides(input).unwrap();

            let mut edges = HashSet::new();
            for i in 0..3 {
                let path = shortest_path(input, &edges, a, b).unwrap();
                for j in 0..path.len() - 1 {
                    let from = path[j];
                    let to = path[j + 1];

                    edges.insert((from, to));
                    let pb = paths_between(input, &edges, a, b);
                    if pb == Some(2 - i) {
                        break;
                    }
                    edges.remove(&(from, to));
                }
            }

            let a_count = count_nodes(input, &edges, a);
            let b_count = count_nodes(input, &edges, b);
            a_count * b_count
        },
        |_input| 0,
    )
}
