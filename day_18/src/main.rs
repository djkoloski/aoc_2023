use std::{
    ops::{Add, Div, Mul},
    str::FromStr,
};

use common::{bail, solve, Context as _, Lines};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Vec2i {
    x: isize,
    y: isize,
}

impl Vec2i {
    const ZERO: Self = Vec2i::new(0, 0);

    const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2i {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn rotate_ccw(self) -> Self {
        match self {
            Direction::Right => Self::Up,
            Direction::Up => Self::Left,
            Direction::Left => Self::Down,
            Direction::Down => Self::Right,
        }
    }

    fn rotate_cw(self) -> Self {
        match self {
            Direction::Right => Self::Down,
            Direction::Up => Self::Right,
            Direction::Left => Self::Up,
            Direction::Down => Self::Left,
        }
    }

    fn apply(self, distance: isize) -> Vec2i {
        match self {
            Self::Right => Vec2i::new(distance, 0),
            Self::Up => Vec2i::new(0, distance),
            Self::Left => Vec2i::new(-distance, 0),
            Self::Down => Vec2i::new(0, -distance),
        }
    }
}

impl FromStr for Direction {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Self::Right,
            "U" => Self::Up,
            "L" => Self::Left,
            "D" => Self::Down,
            _ => bail!("unrecognized direction"),
        })
    }
}

struct HexCode {
    distance: isize,
    direction: Direction,
}

fn hex_to_value(digit: char) -> Result<u8, common::Error> {
    if digit.is_ascii_digit() {
        Ok(digit as u8 - b'0')
    } else if digit as u32 >= 'a' as u32 && digit as u32 <= 'f' as u32 {
        Ok(10 + digit as u8 - b'a')
    } else {
        bail!("invalid hex digit '{digit}'");
    }
}

impl FromStr for HexCode {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix('#')
            .context("expected hex code to begin with '#'")?;
        Ok(Self {
            distance: s
                .chars()
                .take(5)
                .map(hex_to_value)
                .try_fold(0, |a, v| {
                    Result::<_, common::Error>::Ok(a << 4 | v? as isize)
                })?,
            direction: match s.chars().last() {
                None => bail!("expected direction"),
                Some('0') => Direction::Right,
                Some('1') => Direction::Down,
                Some('2') => Direction::Left,
                Some('3') => Direction::Up,
                _ => bail!("unexpected direction instruction"),
            },
        })
    }
}

struct Command {
    direction: Direction,
    distance: isize,
    hex: HexCode,
}

impl FromStr for Command {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(' ');
        Ok(Self {
            direction: pieces.next().context("expected direction")?.parse()?,
            distance: pieces.next().context("expected distance")?.parse()?,
            hex: pieces
                .next()
                .context("expected hex code")?
                .strip_prefix('(')
                .context("expected opening paren")?
                .strip_suffix(')')
                .context("expected closing paren")?
                .parse()?,
        })
    }
}

#[derive(Debug)]
struct Fraction {
    n: isize,
    d: usize,
}

impl Fraction {
    const ZERO: Self = Fraction::new(0, 1);

    const fn new(n: isize, d: usize) -> Self {
        Self { n, d }
    }

    fn reduce(self) -> Self {
        let x = gcd(self.n.unsigned_abs(), self.d);
        Self {
            n: self.n / x as isize,
            d: self.d / x,
        }
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

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = gcd(self.d, rhs.d);
        Self {
            n: self.n * (rhs.d / x) as isize + rhs.n * (self.d / x) as isize,
            d: self.d * rhs.d / x,
        }
        .reduce()
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            n: self.n * rhs.n,
            d: self.d * rhs.d,
        }
        .reduce()
    }
}

impl Div<usize> for Fraction {
    type Output = Fraction;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            n: self.n,
            #[allow(clippy::suspicious_arithmetic_impl)]
            d: self.d * rhs,
        }
        .reduce()
    }
}

fn shoelace(a: Vec2i, b: Vec2i) -> Fraction {
    Fraction::new((a.y + b.y) * (a.x - b.x), 1) / 2
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<Command>| {
            let mut area = Fraction::ZERO;
            let mut p = Vec2i::ZERO;
            for (i, line) in input.lines.iter().enumerate() {
                let add = if line.direction.rotate_cw()
                    == input.lines[(i + 1) % input.lines.len()].direction
                {
                    1
                } else {
                    0
                } + if line.direction.rotate_ccw()
                    == input.lines
                        [(input.lines.len() + i - 1) % input.lines.len()]
                    .direction
                {
                    1
                } else {
                    0
                };

                let n = p + line.direction.apply(line.distance + add - 1);
                area = area + shoelace(p, n);
                p = n;
            }

            assert_eq!(area.d, 1);
            -area.n
        },
        |input| {
            let mut area = Fraction::ZERO;
            let mut p = Vec2i::ZERO;
            for (i, line) in input.lines.iter().enumerate() {
                let add = if line.hex.direction.rotate_cw()
                    == input.lines[(i + 1) % input.lines.len()].hex.direction
                {
                    1
                } else {
                    0
                } + if line.hex.direction.rotate_ccw()
                    == input.lines
                        [(input.lines.len() + i - 1) % input.lines.len()]
                    .hex
                    .direction
                {
                    1
                } else {
                    0
                };

                let n =
                    p + line.hex.direction.apply(line.hex.distance + add - 1);
                area = area + shoelace(p, n);
                p = n;
            }

            assert_eq!(area.d, 1);
            -area.n
        },
    )
}
