use std::{
    env::args,
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    time::Instant,
};

pub use anyhow::Result;

pub trait Input: Sized {
    fn parse_reader<R: BufRead>(reader: R) -> Result<Self>;
}

pub struct Lines<T> {
    pub lines: Vec<T>,
}

impl<T: FromStr> Input for Lines<T>
where
    T::Err: Error + Send + Sync + 'static,
{
    fn parse_reader<R: BufRead>(reader: R) -> Result<Self> {
        let mut lines = Vec::new();

        for line in reader.lines() {
            lines.push(line?.parse::<T>()?);
        }

        Ok(Self { lines })
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
