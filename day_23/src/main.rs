use std::collections::{HashMap, HashSet, VecDeque};

use common::{bail, solve, Grid};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    SlopeRight,
    SlopeUp,
    SlopeLeft,
    SlopeDown,
}

impl Tile {
    fn slope(d: Direction) -> Self {
        match d {
            Direction::Right => Self::SlopeRight,
            Direction::Up => Self::SlopeUp,
            Direction::Left => Self::SlopeLeft,
            Direction::Down => Self::SlopeDown,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = common::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Path,
            '#' => Self::Forest,
            '>' => Self::SlopeRight,
            '^' => Self::SlopeUp,
            '<' => Self::SlopeLeft,
            'v' => Self::SlopeDown,
            _ => bail!("invalid tile: '{value}'"),
        })
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    const ALL: [Self; 4] = [Self::Right, Self::Up, Self::Left, Self::Down];

    fn add(self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Self::Right => x.checked_add(1).map(|x| (x, y)),
            Self::Up => y.checked_sub(1).map(|y| (x, y)),
            Self::Left => x.checked_sub(1).map(|x| (x, y)),
            Self::Down => y.checked_add(1).map(|y| (x, y)),
        }
    }
}

#[derive(Debug)]
struct Edge {
    dest: usize,
    weight: usize,
}

#[derive(Debug)]
struct Node {
    edges: Vec<Edge>,
}

impl Node {
    fn new() -> Self {
        Self { edges: Vec::new() }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn add_node(&mut self) -> usize {
        let result = self.nodes.len();
        self.nodes.push(Node::new());
        result
    }

    fn add_edge(&mut self, src: usize, dest: usize, weight: usize) {
        self.nodes[src].edges.push(Edge { dest, weight })
    }

    fn add_reverse_edges(&mut self) {
        for n in 0..self.nodes.len() {
            for e in 0..self.nodes[n].edges.len() {
                let edge = &self.nodes[n].edges[e];
                let dest = edge.dest;
                let weight = edge.weight;

                if self.nodes[dest].edges.iter().all(|e| e.dest != n) {
                    self.nodes[dest].edges.push(Edge { dest: n, weight });
                }
            }
        }
    }
}

fn trace_corridor(
    grid: &Grid<Tile>,
    mut x: usize,
    mut y: usize,
) -> (usize, usize, usize) {
    let mut px = 0;
    let mut py = 0;
    let mut len = 0;

    while y != grid.height() - 1 {
        for d in Direction::ALL {
            if let Some((nx, ny)) = d.add(x, y) {
                match *grid.get(nx, ny).unwrap() {
                    Tile::Path => {
                        if nx != px || ny != py {
                            px = x;
                            py = y;
                            x = nx;
                            y = ny;
                            len += 1;
                        }
                    }
                    Tile::Forest => (),
                    t => {
                        if t == Tile::slope(d) {
                            let (end_x, end_y) = d.add(nx, ny).unwrap();
                            return (end_x, end_y, len + 2);
                        }
                    }
                }
            }
        }
    }

    (x, y, len)
}

fn to_graph(grid: &Grid<Tile>) -> Graph {
    let mut graph = Graph::new();
    let start_node = graph.add_node();
    let end_node = graph.add_node();

    let mut joints = HashMap::new();

    let mut frontier = VecDeque::new();

    let start_x = (0..grid.width())
        .find(|x| *grid.get(*x, 0).unwrap() == Tile::Path)
        .unwrap();

    frontier.push_back((start_x, 0, start_node));

    while let Some((x, y, src)) = frontier.pop_front() {
        let (end_x, end_y, mut length) = trace_corridor(grid, x, y);

        let dest = if end_y == grid.height() - 1 {
            end_node
        } else if let Some(dest) = joints.get(&(end_x, end_y)) {
            *dest
        } else {
            let dest = graph.add_node();
            joints.insert((end_x, end_y), dest);
            for d in Direction::ALL {
                let (nx, ny) = d.add(end_x, end_y).unwrap();
                if *grid.get(nx, ny).unwrap() == Tile::slope(d) {
                    let (cx, cy) = d.add(nx, ny).unwrap();
                    frontier.push_back((cx, cy, dest));
                }
            }
            dest
        };

        if src != start_node {
            length += 2;
        }

        graph.add_edge(src, dest, length);
    }

    graph
}

fn toposort(graph: &Graph, node: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let mut visited = vec![false; graph.nodes.len()];
    toposort_at(graph, node, &mut visited, &mut result);
    result.reverse();
    result
}

fn toposort_at(
    graph: &Graph,
    node: usize,
    visited: &mut Vec<bool>,
    result: &mut Vec<usize>,
) {
    if !visited[node] {
        visited[node] = true;
        for e in graph.nodes[node].edges.iter() {
            toposort_at(graph, e.dest, visited, result);
        }
        result.push(node);
    }
}

fn longest_path_dag(graph: &Graph) -> usize {
    let mut max_distance = HashMap::new();
    let order = toposort(graph, 0);

    max_distance.insert(0, 0);
    for &node in order.iter() {
        let d = max_distance[&node];
        for e in graph.nodes[node].edges.iter() {
            let x = max_distance.entry(e.dest).or_insert(d + e.weight);
            *x = usize::max(*x, d + e.weight);
        }
    }

    max_distance[&1]
}

fn longest_path(graph: &Graph) -> usize {
    let mut visited = HashSet::new();
    longest_path_sub(graph, 0, &mut visited).unwrap()
}

fn longest_path_sub(
    graph: &Graph,
    node: usize,
    visited: &mut HashSet<usize>,
) -> Option<usize> {
    if node == 1 {
        Some(0)
    } else if !visited.contains(&node) {
        visited.insert(node);

        let mut max = None;
        for e in graph.nodes[node].edges.iter() {
            if let Some(d) = longest_path_sub(graph, e.dest, visited) {
                max = Some(usize::max(max.unwrap_or(0), e.weight + d));
            }
        }

        visited.remove(&node);

        max
    } else {
        None
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Grid<Tile>| longest_path_dag(&to_graph(input)),
        |input| {
            let mut graph = to_graph(input);
            graph.add_reverse_edges();
            longest_path(&graph)
        },
    )
}
