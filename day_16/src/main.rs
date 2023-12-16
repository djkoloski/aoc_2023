use std::ops::{BitAnd, BitOr};

use common::{bail, solve, Grid};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Empty,
    SplitHorizontal,
    SplitVertical,
    ReflectForward,
    ReflectBackward,
}

impl TryFrom<char> for Tile {
    type Error = common::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '-' => Self::SplitHorizontal,
            '|' => Self::SplitVertical,
            '/' => Self::ReflectForward,
            '\\' => Self::ReflectBackward,
            _ => bail!("invalid char '{value}'"),
        })
    }
}

impl Tile {
    fn unrotate(self, d: Direction) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::SplitHorizontal => match d {
                Direction::Right | Direction::Left => Self::SplitHorizontal,
                Direction::Up | Direction::Down => Self::SplitVertical,
            },
            Self::SplitVertical => match d {
                Direction::Right | Direction::Left => Self::SplitVertical,
                Direction::Up | Direction::Down => Self::SplitHorizontal,
            },
            Self::ReflectForward => match d {
                Direction::Right | Direction::Left => Self::ReflectForward,
                Direction::Up | Direction::Down => Self::ReflectBackward,
            },
            Self::ReflectBackward => match d {
                Direction::Right | Direction::Left => Self::ReflectBackward,
                Direction::Up | Direction::Down => Self::ReflectForward,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Copy, Default)]
struct Energized(u8);

impl Energized {
    fn is_some(self) -> bool {
        self.0 != 0
    }
}

impl BitAnd<Direction> for Energized {
    type Output = bool;

    fn bitand(self, rhs: Direction) -> Self::Output {
        (self.0 & 1 << rhs as u8) != 0
    }
}

impl BitOr<Direction> for Energized {
    type Output = Self;

    fn bitor(self, rhs: Direction) -> Self::Output {
        Self(self.0 | 1 << rhs as u8)
    }
}

fn propagate(
    input: &Grid<Tile>,
    ix: usize,
    iy: usize,
    id: Direction,
) -> Grid<Energized> {
    let mut result = Grid::default(input.width(), input.height());

    let mut queue = vec![(ix, iy, id)];
    while let Some((x, y, d)) = queue.pop() {
        if *result.get(x, y).unwrap() & d {
            continue;
        }

        result.set(x, y, *result.get(x, y).unwrap() | d);

        match input.get(x, y).unwrap().unrotate(d) {
            Tile::Empty | Tile::SplitHorizontal => {
                enqueue(input, &mut queue, x, y, d)
            }
            Tile::SplitVertical => {
                enqueue(input, &mut queue, x, y, d.rotate_ccw());
                enqueue(input, &mut queue, x, y, d.rotate_cw());
            }
            Tile::ReflectForward => {
                enqueue(input, &mut queue, x, y, d.rotate_cw())
            }
            Tile::ReflectBackward => {
                enqueue(input, &mut queue, x, y, d.rotate_ccw())
            }
        }
    }

    result
}

fn enqueue(
    input: &Grid<Tile>,
    queue: &mut Vec<(usize, usize, Direction)>,
    x: usize,
    y: usize,
    d: Direction,
) {
    if let Some((x, y)) = d.offset(x, y) {
        if x < input.width() && y < input.height() {
            queue.push((x, y, d));
        }
    }
}

fn count_energized(
    input: &Grid<Tile>,
    ix: usize,
    iy: usize,
    id: Direction,
) -> usize {
    let energized = propagate(input, ix, iy, id);
    energized
        .iter()
        .filter(|(x, y)| energized.get(*x, *y).unwrap().is_some())
        .count()
}

fn main() -> common::Result<()> {
    solve(
        |input: &Grid<Tile>| count_energized(input, 0, 0, Direction::Right),
        |input: &Grid<Tile>| {
            let mut max = 0;

            for x in 0..input.width() {
                max = usize::max(
                    max,
                    count_energized(input, x, 0, Direction::Up),
                );
                max = usize::max(
                    max,
                    count_energized(
                        input,
                        x,
                        input.height() - 1,
                        Direction::Down,
                    ),
                );
            }

            for y in 0..input.height() {
                max = usize::max(
                    max,
                    count_energized(input, 0, y, Direction::Right),
                );
                max = usize::max(
                    max,
                    count_energized(
                        input,
                        input.width() - 1,
                        y,
                        Direction::Left,
                    ),
                );
            }

            max
        },
    )
}
