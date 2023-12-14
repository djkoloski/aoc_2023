use std::collections::HashMap;

use common::{bail, solve, Grid};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Tile {
    Empty,
    Square,
    Round,
}

impl TryFrom<char> for Tile {
    type Error = common::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '#' => Self::Square,
            'O' => Self::Round,
            _ => bail!("invalid tile type: {value}"),
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
    const fn offset_x(self) -> isize {
        match self {
            Direction::Right => 1,
            Direction::Left => -1,
            Direction::Up | Direction::Down => 0,
        }
    }

    const fn offset_y(self) -> isize {
        match self {
            Direction::Down => 1,
            Direction::Up => -1,
            Direction::Right | Direction::Left => 0,
        }
    }
}

fn tilt(grid: &mut Grid<Tile>, d: Direction) {
    let mut moved = true;
    while moved {
        moved = false;
        for (x, y) in grid.iter() {
            if let (Some(dx), Some(dy)) = (
                x.checked_add_signed(d.offset_x()),
                y.checked_add_signed(d.offset_y()),
            ) {
                if *grid.get(x, y).unwrap() == Tile::Round
                    && dx < grid.width()
                    && dy < grid.height()
                    && *grid.get(dx, dy).unwrap() == Tile::Empty
                {
                    grid.set(x, y, Tile::Empty);
                    grid.set(dx, dy, Tile::Round);
                    moved = true;
                }
            }
        }
    }
}

fn load(grid: &Grid<Tile>) -> usize {
    let mut total = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if *grid.get(x, y).unwrap() == Tile::Round {
                total += grid.height() - y;
            }
        }
    }
    total
}

fn spin_cycle(grid: &mut Grid<Tile>) {
    tilt(grid, Direction::Up);
    tilt(grid, Direction::Left);
    tilt(grid, Direction::Down);
    tilt(grid, Direction::Right);
}

fn main() -> common::Result<()> {
    solve(
        |input: &Grid<Tile>| {
            let mut grid = input.clone();
            tilt(&mut grid, Direction::Up);
            load(&grid)
        },
        |input| {
            const CYCLE_COUNT: usize = 1_000_000_000;

            let mut grid = input.clone();
            let mut seen = HashMap::new();
            seen.insert(grid.clone(), 0);

            let mut cycle = 0;
            while cycle < CYCLE_COUNT {
                spin_cycle(&mut grid);
                cycle += 1;

                if let Some(last_seen) = seen.get(&grid) {
                    // Jump ahead
                    let cycle_length = cycle - last_seen;
                    let remaining = CYCLE_COUNT - cycle;
                    cycle += remaining / cycle_length * cycle_length;
                } else {
                    seen.insert(grid.clone(), cycle);
                }
            }

            load(&grid)
        },
    )
}
