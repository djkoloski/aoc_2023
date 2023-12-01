use std::{
    env::args,
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    time::Instant,
};

use anyhow::Result;

pub trait Input: Sized {
    fn parse_reader<R: BufRead>(reader: R) -> Result<Self>;
}

struct Lines<T> {
    lines: Vec<T>,
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

fn solve<I, P1, O1, P2, O2>(part_one: P1, part_two: P2) -> Result<()>
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
    let file = File::open(&path).expect("unable to open input file");
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

const NAMES: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> Result<()> {
    solve(
        |input: &Lines<String>| {
            input
                .lines
                .iter()
                .map(|line| {
                    let mut digits = line
                        .chars()
                        .filter(char::is_ascii_digit)
                        .map(|c| c.to_digit(10).unwrap());
                    let first = digits.next().unwrap();
                    let last = digits.last().unwrap_or(first);
                    first * 10 + last
                })
                .sum::<u32>()
        },
        |input| {
            input
                .lines
                .iter()
                .map(|line| {
                    let mut digits = line
                        .chars()
                        .enumerate()
                        .filter(|(_, c)| c.is_ascii_digit())
                        .map(|(i, c)| (i, c.to_digit(10).unwrap()));
                    let mut first = digits.next().unwrap();
                    let mut last = digits.last().unwrap_or(first);

                    for (i, name) in NAMES.iter().enumerate() {
                        if let Some(index) = line.find(name) {
                            if index < first.0 {
                                first.0 = index;
                                first.1 = i as u32 + 1;
                            }
                        }
                        if let Some(index) = line.rfind(name) {
                            if index > last.0 {
                                last.0 = index;
                                last.1 = i as u32 + 1;
                            }
                        }
                    }

                    first.1 * 10 + last.1
                })
                .sum::<u32>()
        },
    )
}
