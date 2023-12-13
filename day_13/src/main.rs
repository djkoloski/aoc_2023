use std::io::prelude::BufRead;

use common::{bail, solve, Grid, Input};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Ash,
    Rocks,
}

impl TryFrom<char> for Tile {
    type Error = common::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Ash,
            '#' => Tile::Rocks,
            _ => bail!("invalid tile character '{value}'"),
        })
    }
}

struct Grids {
    grids: Vec<Grid<Tile>>,
}

impl Input for Grids {
    fn parse_reader<R: BufRead>(reader: R) -> common::Result<Self> {
        let mut grids = Vec::new();

        let mut width = 0;
        let mut height = 0;
        let mut current = Vec::new();
        for line in reader.lines() {
            let line = line?;

            if line.is_empty() {
                let mut elements = Vec::new();
                core::mem::swap(&mut current, &mut elements);

                grids.push(Grid::from_elements(width, height, elements));

                width = 0;
                height = 0;
            } else {
                if width == 0 {
                    width = line.chars().count();
                }
                height += 1;
                for c in line.chars() {
                    current.push(Tile::try_from(c)?);
                }
            }
        }

        if width != 0 {
            grids.push(Grid::from_elements(width, height, current));
        }

        Ok(Self { grids })
    }
}

fn reflected_vertical(grid: &Grid<Tile>, smudges: usize) -> Option<usize> {
    'outer: for x in 1..grid.width() {
        let mut s = 0;
        for i in 0..usize::min(x, grid.width() - x) {
            let lx = x - i - 1;
            let rx = x + i;
            for y in 0..grid.height() {
                if grid.get(lx, y) != grid.get(rx, y) {
                    s += 1;
                    if s > smudges {
                        continue 'outer;
                    }
                }
            }
        }
        if s == smudges {
            return Some(x);
        }
    }
    None
}

fn reflected_horizontal(grid: &Grid<Tile>, smudges: usize) -> Option<usize> {
    'outer: for y in 1..grid.height() {
        let mut s = 0;
        for i in 0..usize::min(y, grid.height() - y) {
            let ly = y - i - 1;
            let ry = y + i;
            for x in 0..grid.width() {
                if grid.get(x, ly) != grid.get(x, ry) {
                    s += 1;
                    if s > smudges {
                        continue 'outer;
                    }
                }
            }
        }
        if s == smudges {
            return Some(y);
        }
    }
    None
}

fn main() -> common::Result<()> {
    solve(
        |input: &Grids| {
            input
                .grids
                .iter()
                .map(|g| {
                    reflected_vertical(g, 0)
                        .or_else(|| reflected_horizontal(g, 0).map(|x| x * 100))
                        .unwrap()
                })
                .sum::<usize>()
        },
        |input: &Grids| {
            input
                .grids
                .iter()
                .map(|g| {
                    reflected_vertical(g, 1)
                        .or_else(|| reflected_horizontal(g, 1).map(|x| x * 100))
                        .unwrap()
                })
                .sum::<usize>()
        },
    )
}
