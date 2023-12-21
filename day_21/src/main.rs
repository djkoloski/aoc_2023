use std::collections::VecDeque;

use common::{bail, solve, Grid};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Tile {
    Start,
    #[default]
    Garden,
    Rock,
}

impl Tile {
    fn is_passable(self) -> bool {
        match self {
            Self::Start | Self::Garden => true,
            Self::Rock => false,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = common::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'S' => Self::Start,
            '.' => Self::Garden,
            '#' => Self::Rock,
            _ => bail!("unrecognized character '{value}'"),
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
            Self::Up => y.checked_add(1).map(|y| (x, y)),
            Self::Left => x.checked_sub(1).map(|x| (x, y)),
            Self::Down => y.checked_sub(1).map(|y| (x, y)),
        }
    }
}

fn reachable(grid: &Grid<Tile>, steps: usize) -> usize {
    let mut dist = Grid::<Option<usize>>::default(grid.width(), grid.height());
    let mut frontier = VecDeque::new();

    let (start_x, start_y) = grid
        .iter()
        .find(|&(x, y)| *grid.get(x, y).unwrap() == Tile::Start)
        .unwrap();
    frontier.push_back((start_x, start_y, 0));

    while let Some((x, y, d)) = frontier.pop_front() {
        if dist.get(x, y).unwrap().is_some_and(|n| n <= d) {
            continue;
        }

        dist.set(x, y, Some(d));

        for r in Direction::ALL {
            match r.add(x, y) {
                Some((nx, ny))
                    if nx < grid.width()
                        && ny < grid.height()
                        && grid.get(nx, ny).unwrap().is_passable() =>
                {
                    frontier.push_back((nx, ny, d + 1));
                }
                _ => (),
            }
        }
    }

    dist.iter()
        .filter(|&(x, y)| {
            let d = dist.get(x, y).unwrap();
            d.is_some_and(|d| d <= steps && d % 2 == steps % 2)
        })
        .count()
}

fn main() -> common::Result<()> {
    solve(
        |input: &Grid<Tile>| reachable(input, 64),
        |input| {
            let mut mega = Grid::default(input.width() * 5, input.height() * 5);
            for rx in 0..5 {
                for ry in 0..5 {
                    for (x, y) in input.iter() {
                        let tile = if *input.get(x, y).unwrap() == Tile::Rock {
                            Tile::Rock
                        } else {
                            Tile::Garden
                        };

                        mega.set(
                            x + rx * input.width(),
                            y + ry * input.height(),
                            tile,
                        );
                    }
                }
            }

            mega.set(
                2 * input.width() + (input.width() - 1) / 2,
                2 * input.height() + (input.height() - 1) / 2,
                Tile::Start,
            );

            let p0 = reachable(&mega, (input.width() - 1) / 2);
            let p1 = reachable(&mega, input.width() + (input.width() - 1) / 2);
            let p2 =
                reachable(&mega, 2 * input.width() + (input.width() - 1) / 2);

            // Manually solving quadratics. What a shitshow.
            let n = (26501365 - (input.width() - 1) / 2) / input.width();
            let c1 = p0;
            let c0_2e = p1 - 4 * c1;
            let c0 = p2 - 9 * c1 - 3 * c0_2e;
            let e0_e1 = (p1 - c0 - 4 * c1) / 2;

            // 🤡
            assert!(n % 2 == 0);
            usize::pow(2 * (n / 2) + 1, 2) * c1
                + usize::pow(2 * ((n + 1) / 2), 2) * c0
                + n * (n + 1) * e0_e1
        },
    )
}
