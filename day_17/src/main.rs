use std::collections::VecDeque;

use common::{solve, Grid};

struct Tile {
    heat_loss: usize,
}

impl TryFrom<char> for Tile {
    type Error = common::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Self {
            heat_loss: value as usize - '0' as usize,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn rotate_ccw(self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }

    fn rotate_cw(self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }

    fn offset(self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Right => x.checked_add(1).map(|x| (x, y)),
            Direction::Up => y.checked_add(1).map(|y| (x, y)),
            Direction::Left => x.checked_sub(1).map(|x| (x, y)),
            Direction::Down => y.checked_sub(1).map(|y| (x, y)),
        }
    }
}

const MAX_TIME: usize = 10;

#[derive(Default)]
struct Node {
    min_costs: [[Option<usize>; MAX_TIME]; 4],
}

fn min_cost(input: &Grid<Tile>, min_time: usize, max_time: usize) -> usize {
    let mut nodes = Grid::<Node>::default(input.width(), input.height());
    let mut frontier = VecDeque::new();

    explore(
        input,
        &mut nodes,
        &mut frontier,
        0,
        0,
        0,
        Direction::Right,
        0,
    );
    explore(input, &mut nodes, &mut frontier, 0, 0, 0, Direction::Up, 0);

    while let Some((c, x, y, d, t)) = frontier.pop_front() {
        if nodes.get(x, y).unwrap().min_costs[d as usize][t]
            .as_ref()
            .is_some_and(|m| *m < c)
        {
            continue;
        }

        if t + 1 < max_time {
            explore(input, &mut nodes, &mut frontier, c, x, y, d, t + 1);
        }
        if t >= min_time {
            explore(
                input,
                &mut nodes,
                &mut frontier,
                c,
                x,
                y,
                d.rotate_cw(),
                0,
            );
            explore(
                input,
                &mut nodes,
                &mut frontier,
                c,
                x,
                y,
                d.rotate_ccw(),
                0,
            );
        }
    }

    nodes
        .get(input.width() - 1, input.height() - 1)
        .unwrap()
        .min_costs
        .iter()
        .flat_map(|x| x[min_time..].iter().filter_map(|n| *n))
        .min()
        .unwrap()
}

// I am too done with this problem to address this lint
#[allow(clippy::too_many_arguments)]
fn explore(
    input: &Grid<Tile>,
    nodes: &mut Grid<Node>,
    frontier: &mut VecDeque<(usize, usize, usize, Direction, usize)>,
    c: usize,
    x: usize,
    y: usize,
    d: Direction,
    nt: usize,
) {
    if let Some((nx, ny)) = d.offset(x, y) {
        if nx >= input.width() || ny >= input.height() {
            return;
        }

        let best =
            &mut nodes.get_mut(nx, ny).unwrap().min_costs[d as usize][nt];
        let current = c + input.get(nx, ny).unwrap().heat_loss;
        if best.as_ref().is_some_and(|n| *n <= current) {
            return;
        }

        *best = Some(current);
        frontier.push_back((current, nx, ny, d, nt));
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Grid<Tile>| min_cost(input, 0, 3),
        |input| min_cost(input, 3, 10),
    )
}
