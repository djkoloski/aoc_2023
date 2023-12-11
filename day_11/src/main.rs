use common::{bail, solve, Grid};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Galaxy,
}

impl TryFrom<char> for Space {
    type Error = common::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Space::Empty,
            '#' => Space::Galaxy,
            _ => bail!("expected '.' or '#', found '{value}'"),
        })
    }
}

fn galaxy_coords(grid: &Grid<Space>, scale: usize) -> Vec<(usize, usize)> {
    let mut coords = grid
        .iter()
        .filter(|(x, y)| *grid.get(*x, *y).unwrap() == Space::Galaxy)
        .collect::<Vec<_>>();

    for _ in 0..2 {
        coords.sort_by_key(|(x, _)| *x);
        let mut expansion = 0;
        let mut prev_coord = coords[0].0;
        for (coord, _) in coords.iter_mut().skip(1) {
            let new_prev = *coord;
            expansion += coord.saturating_sub(prev_coord + 1) * scale;
            *coord += expansion;
            prev_coord = new_prev;
        }
        coords.iter_mut().for_each(|(x, y)| core::mem::swap(x, y));
    }

    coords
}

fn main() -> common::Result<()> {
    solve(
        |input: &Grid<Space>| {
            let coords = galaxy_coords(input, 1);
            let mut total = 0;
            for i in 0..coords.len() {
                for j in i + 1..coords.len() {
                    total += coords[i].0.abs_diff(coords[j].0)
                        + coords[i].1.abs_diff(coords[j].1);
                }
            }
            total
        },
        |input| {
            let coords = galaxy_coords(input, 999_999);
            let mut total = 0;
            for i in 0..coords.len() {
                for j in i + 1..coords.len() {
                    total += coords[i].0.abs_diff(coords[j].0)
                        + coords[i].1.abs_diff(coords[j].1);
                }
            }
            total
        },
    )
}
