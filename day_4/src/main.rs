use std::str::FromStr;

use common::{solve, Context, Lines};

struct Card {
    winning: Vec<usize>,
    have: Vec<usize>,
}

impl Card {
    fn matches(&self) -> usize {
        self.have
            .iter()
            .filter(|x| self.winning.contains(x))
            .count()
    }
}

impl FromStr for Card {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_card, rest) = s.split_once(": ").context("expected colon")?;
        let (winning, have) =
            rest.split_once(" | ").context("expected vertical bar")?;
        Ok(Self {
            winning: winning
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(str::parse)
                .collect::<Result<_, _>>()?,
            have: have
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        })
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<Card>| {
            input
                .lines
                .iter()
                .map(|card| match card.matches() {
                    0 => 0,
                    x => 1 << (x - 1),
                })
                .sum::<usize>()
        },
        |input| {
            let mut copies = vec![1; input.lines.len()];
            for i in 0..input.lines.len() {
                let matches = input.lines[i].matches();
                for j in (i + 1)..usize::min(i + matches + 1, input.lines.len())
                {
                    copies[j] += copies[i];
                }
            }

            copies.iter().sum::<usize>()
        },
    )
}
