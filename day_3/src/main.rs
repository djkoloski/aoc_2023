use common::{bail, solve, Grid};

enum TileKind {
    Empty,
    Digit(u8),
    Symbol(char),
}

struct Tile {
    inner: u8,
}

impl Tile {
    fn kind(&self) -> TileKind {
        if self.inner == b'.' {
            TileKind::Empty
        } else if self.inner >= b'0' && self.inner <= b'9' {
            TileKind::Digit(self.inner - b'0')
        } else {
            TileKind::Symbol(self.inner as char)
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = common::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_digit() || value.is_ascii_punctuation() {
            Ok(Self { inner: value as u8 })
        } else {
            bail!("invalid character")
        }
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Grid<Tile>| {
            let mut total = 0;
            for y in 0..input.height() {
                let mut n = None;
                let mut is_adjacent = false;

                for x in 0..input.width() {
                    if let TileKind::Digit(d) = input.get(x, y).unwrap().kind()
                    {
                        n = Some(n.map_or(d as i32, |n| n * 10 + d as i32));
                        is_adjacent = is_adjacent
                            || input.adjacent(x, y).any(|(nx, ny)| {
                                matches!(
                                    input.get(nx, ny).unwrap().kind(),
                                    TileKind::Symbol(_)
                                )
                            });
                    } else if let Some(part) = n {
                        if is_adjacent {
                            total += part;
                            is_adjacent = false;
                        }
                        n = None;
                    }
                }

                if is_adjacent && n.is_some() {
                    total += n.unwrap();
                }
            }

            total
        },
        |input| {
            let mut starts = Grid::default(input.width(), input.height());

            for y in 0..input.height() {
                let mut start = 0;
                for x in 0..input.width() {
                    if !matches!(
                        input.get(x, y).unwrap().kind(),
                        TileKind::Digit(_)
                    ) {
                        start = x + 1;
                    }
                    starts.set(x, y, start);
                }
            }

            let mut total = 0;
            for (x, y) in input.iter() {
                if matches!(
                    input.get(x, y).unwrap().kind(),
                    TileKind::Symbol('*')
                ) {
                    let mut unique = input
                        .adjacent(x, y)
                        .filter(|(nx, ny)| {
                            matches!(
                                input.get(*nx, *ny).unwrap().kind(),
                                TileKind::Digit(_)
                            )
                        })
                        .map(|(nx, ny)| (starts.get(nx, ny).unwrap(), ny))
                        .collect::<Vec<_>>();
                    unique.dedup();
                    if unique.len() == 2 {
                        let mut product = 1;
                        for (sx, sy) in unique.iter() {
                            let mut number = 0;
                            for x in **sx.. {
                                if let Some(TileKind::Digit(d)) =
                                    input.get(x, *sy).map(Tile::kind)
                                {
                                    number = number * 10 + d as usize;
                                } else {
                                    break;
                                }
                            }
                            product *= number;
                        }
                        total += product;
                    }
                }
            }

            total
        },
    )
}
