use common::{bail, solve, Grid};

const RIGHT_FLAG: isize = 0b0001;
const UP_FLAG: isize = 0b0010;
const LEFT_FLAG: isize = 0b0100;
const DOWN_FLAG: isize = 0b1000;
const FLAG_MASK: isize = RIGHT_FLAG | UP_FLAG | LEFT_FLAG | DOWN_FLAG;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right = RIGHT_FLAG,
    Up = UP_FLAG,
    Left = LEFT_FLAG,
    Down = DOWN_FLAG,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Right,
        Direction::Up,
        Direction::Left,
        Direction::Down,
    ];

    fn reverse(self) -> Self {
        match self {
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
        }
    }

    fn add(self, x: usize, y: usize) -> Option<(usize, usize)> {
        let (dx, dy) = match self {
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
        };
        x.checked_add_signed(dx)
            .and_then(|x| y.checked_add_signed(dy).map(|y| (x, y)))
    }
}

const VERTICAL_FLAG: isize = UP_FLAG | DOWN_FLAG;
const HORIZONTAL_FLAG: isize = RIGHT_FLAG | LEFT_FLAG;
const ELBOW_NE_FLAG: isize = RIGHT_FLAG | UP_FLAG;
const ELBOW_NW_FLAG: isize = UP_FLAG | LEFT_FLAG;
const ELBOW_SW_FLAG: isize = LEFT_FLAG | DOWN_FLAG;
const ELBOW_SE_FLAG: isize = RIGHT_FLAG | DOWN_FLAG;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vertical = VERTICAL_FLAG,
    Horizontal = HORIZONTAL_FLAG,
    ElbowNE = ELBOW_NE_FLAG,
    ElbowNW = ELBOW_NW_FLAG,
    ElbowSW = ELBOW_SW_FLAG,
    ElbowSE = ELBOW_SE_FLAG,
    Ground = 0,
    Start = FLAG_MASK + 1,
}

impl Tile {
    fn from_flags(flags: isize) -> Option<Self> {
        Some(match flags {
            VERTICAL_FLAG => Self::Vertical,
            HORIZONTAL_FLAG => Self::Horizontal,
            ELBOW_NE_FLAG => Self::ElbowNE,
            ELBOW_NW_FLAG => Self::ElbowNW,
            ELBOW_SW_FLAG => Self::ElbowSW,
            ELBOW_SE_FLAG => Self::ElbowSE,
            _ => return None,
        })
    }

    fn directions(&self) -> Directions {
        Directions(*self as u8)
    }
}

struct Directions(u8);

impl Iterator for Directions {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.0.trailing_zeros();
        if i > 3 {
            None
        } else {
            self.0 &= !(1 << i);
            Some(match i {
                0 => Direction::Right,
                1 => Direction::Up,
                2 => Direction::Left,
                3 => Direction::Down,
                _ => unreachable!(),
            })
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = common::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::ElbowNE,
            'J' => Self::ElbowNW,
            '7' => Self::ElbowSW,
            'F' => Self::ElbowSE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => bail!("unexpected grid character '{value}'"),
        })
    }
}

fn replace_start(grid: &mut Grid<Tile>) -> (usize, usize) {
    let (start_x, start_y) = grid
        .iter()
        .find(|&(x, y)| *grid.get(x, y).unwrap() == Tile::Start)
        .unwrap();

    let mut start_tile = 0;

    for d in Direction::ALL {
        if let Some((nx, ny)) = d.add(start_x, start_y) {
            if let Some(tile) = grid.get(nx, ny) {
                if tile.directions().any(|dir| dir == d.reverse()) {
                    start_tile |= d as isize;
                }
            }
        }
    }

    grid.set(start_x, start_y, Tile::from_flags(start_tile).unwrap());

    (start_x, start_y)
}

fn mark_loop(
    grid: &Grid<Tile>,
    start_x: usize,
    start_y: usize,
) -> (Grid<bool>, usize) {
    let mut result = Grid::default(grid.width(), grid.height());

    let mut length = 0;
    let mut x = start_x;
    let mut y = start_y;

    while !result.get(x, y).unwrap() {
        result.set(x, y, true);
        length += 1;

        for d in grid.get(x, y).unwrap().directions() {
            let (nx, ny) = d.add(x, y).unwrap();
            if !result.get(nx, ny).unwrap() {
                x = nx;
                y = ny;
                break;
            }
        }
    }

    (result, length)
}

fn main() -> common::Result<()> {
    solve(
        |input: &Grid<Tile>| {
            let mut input = input.clone();
            let (start_x, start_y) = replace_start(&mut input);

            mark_loop(&input, start_x, start_y).1 / 2
        },
        |input| {
            let mut input = input.clone();
            let (start_x, start_y) = replace_start(&mut input);

            let visited = mark_loop(&input, start_x, start_y).0;

            let mut total = 0;
            for y in 0..input.height() {
                let mut inside = false;
                let mut entered_top = false;
                for x in 0..input.width() {
                    if *visited.get(x, y).unwrap() {
                        // This tile is on the loop
                        match input.get(x, y).unwrap() {
                            Tile::Horizontal | Tile::Ground | Tile::Start => (),
                            Tile::Vertical => inside = !inside,
                            Tile::ElbowNE => entered_top = true,
                            Tile::ElbowNW => {
                                if !entered_top {
                                    inside = !inside;
                                }
                            }
                            Tile::ElbowSE => entered_top = false,
                            Tile::ElbowSW => {
                                if entered_top {
                                    inside = !inside;
                                }
                            }
                        }
                    } else if inside {
                        total += 1;
                    }
                }
            }

            total
        },
    )
}
