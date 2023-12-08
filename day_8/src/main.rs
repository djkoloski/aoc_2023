use std::{collections::HashMap, io::prelude::BufRead};

use common::{solve, Context, Input};

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct Node {
    left: String,
    right: String,
}

impl Node {
    fn get(&self, d: Direction) -> &str {
        match d {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

struct Map {
    steps: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

impl Map {
    fn find_z(&self, start: &str) -> usize {
        let mut current = start;
        let mut steps = 0;

        loop {
            current =
                self.nodes[current].get(self.steps[steps % self.steps.len()]);
            steps += 1;

            if current.ends_with('Z') {
                break steps;
            }
        }
    }
}

impl Input for Map {
    fn parse_reader<R: BufRead>(reader: R) -> common::Result<Self> {
        let mut lines = reader.lines();

        let steps = lines
            .next()
            .context("expected steps")??
            .chars()
            .map(|c| {
                if c == 'L' {
                    Direction::Left
                } else {
                    Direction::Right
                }
            })
            .collect();

        lines.next().context("expected newline")??;

        let mut nodes = HashMap::new();

        for line in lines {
            let line = line?;
            let (name, rest) =
                line.split_once(" = ").context("expected name and nodes")?;
            let (left, right) =
                rest.split_once(", ").context("expected left and right")?;
            nodes.insert(
                name.to_string(),
                Node {
                    left: left
                        .strip_prefix('(')
                        .context("missing open paren")?
                        .to_string(),
                    right: right
                        .strip_suffix(')')
                        .context("missing close paren")?
                        .to_string(),
                },
            );
        }

        Ok(Self { steps, nodes })
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() -> common::Result<()> {
    solve(
        |input: &Map| input.find_z("AAA"),
        |input| {
            let start_nodes = input
                .nodes
                .keys()
                .filter(|name| name.ends_with('A'))
                .map(|name| name.as_str())
                .collect::<Vec<_>>();
            start_nodes
                .iter()
                .map(|start| input.find_z(start))
                .fold(1, lcm)
        },
    )
}
