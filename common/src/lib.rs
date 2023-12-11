use std::{
    env::args,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    time::Instant,
};

pub use anyhow::{bail, Context, Error, Result};

pub trait Input: Sized {
    fn parse_reader<R: BufRead>(reader: R) -> Result<Self>;
}

pub struct Lines<T> {
    pub lines: Vec<T>,
}

impl<T: FromStr> Input for Lines<T>
where
    Error: From<T::Err>,
{
    fn parse_reader<R: BufRead>(reader: R) -> Result<Self> {
        let mut lines = Vec::new();

        for line in reader.lines() {
            lines.push(line?.parse::<T>()?);
        }

        Ok(Self { lines })
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    elements: Vec<T>,
}

impl<T> Grid<T> {
    pub fn default(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self {
            width,
            height,
            elements: (0..width * height).map(|_| T::default()).collect(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.elements[x + y * self.width])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        assert!(x < self.width && y < self.height);
        self.elements[x + y * self.width] = value;
    }

    pub fn iter(&self) -> Region {
        Region::new(0, 0, self.width(), self.height())
    }

    pub fn adjacent(&self, x: usize, y: usize) -> Region {
        self.neighborhood(x, y, 1)
    }

    pub fn neighborhood(&self, x: usize, y: usize, radius: usize) -> Region {
        let lx = x.saturating_sub(radius);
        let ly = y.saturating_sub(radius);
        let r = radius.saturating_add(1);
        let ux = usize::min(x.saturating_add(r), self.width());
        let uy = usize::min(y.saturating_add(r), self.height());

        Region::new(lx, ly, ux - lx, uy - ly)
    }
}

pub struct Region {
    x: usize,
    y: usize,
    width: usize,
    area: usize,
    index: usize,
}

impl Region {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            area: width * height,
            index: 0,
        }
    }
}

impl Iterator for Region {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.area {
            None
        } else {
            let result = Some((
                self.x + self.index % self.width,
                self.y + self.index / self.width,
            ));

            self.index += 1;

            result
        }
    }
}

impl<T> Input for Grid<T>
where
    T: TryFrom<char>,
    Error: From<T::Error>,
{
    fn parse_reader<R: BufRead>(reader: R) -> Result<Self> {
        let mut width = None;
        let mut height = 0;
        let mut elements = Vec::new();

        for line in reader.lines() {
            let line = line?;

            if width.is_some_and(|w| w != line.len()) {
                bail!("uneven grid lines");
            }
            width = Some(line.len());
            height += 1;

            for c in line.chars() {
                elements.push(T::try_from(c)?);
            }
        }

        Ok(Self {
            width: width.context("empty grid input")?,
            height,
            elements,
        })
    }
}

pub fn solve<I, P1, O1, P2, O2>(part_one: P1, part_two: P2) -> Result<()>
where
    I: Input,
    P1: FnOnce(&I) -> O1,
    O1: Display,
    P2: FnOnce(&I) -> O2,
    O2: Display,
{
    let path = args()
        .nth(1)
        .expect("expected input path as first argument");
    let file = File::open(path).expect("unable to open input file");
    let input = I::parse_reader(BufReader::new(file))?;

    let start = Instant::now();
    let solution = part_one(&input);
    println!(
        "Solved part one in {} seconds",
        start.elapsed().as_secs_f32()
    );
    println!("{solution}");

    let start = Instant::now();
    let solution = part_two(&input);
    println!(
        "Solved part two in {} seconds",
        start.elapsed().as_secs_f32()
    );
    println!("{solution}");

    Ok(())
}
