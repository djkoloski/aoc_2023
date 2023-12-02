use core::str::FromStr;

use common::{bail, solve, Context, Lines};

struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, rounds) = s.split_once(": ").context("expected colon")?;
        Ok(Self {
            id: game
                .strip_prefix("Game ")
                .context("expected Game")?
                .parse()?,
            rounds: rounds
                .split("; ")
                .map(Round::from_str)
                .collect::<common::Result<Vec<_>>>()?,
        })
    }
}

#[derive(Default)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl Round {
    fn max(a: Self, b: &Self) -> Self {
        Self {
            red: usize::max(a.red, b.red),
            green: usize::max(a.green, b.green),
            blue: usize::max(a.blue, b.blue),
        }
    }
}

impl FromStr for Round {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self {
            red: 0,
            green: 0,
            blue: 0,
        };

        for set in s.split(", ") {
            let mut pieces = set.split(' ');
            let count =
                pieces.next().context("expected count")?.parse::<usize>()?;
            let color = pieces.next().context("expected color")?;

            match color {
                "red" => result.red += count,
                "green" => result.green += count,
                "blue" => result.blue += count,
                _ => bail!("invalid color"),
            }
        }

        Ok(result)
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<Game>| {
            input
                .lines
                .iter()
                .filter(|game| {
                    let max =
                        game.rounds.iter().fold(Round::default(), Round::max);
                    max.red <= 12 && max.green <= 13 && max.blue <= 14
                })
                .map(|g| g.id)
                .sum::<usize>()
        },
        |input: &Lines<Game>| {
            input
                .lines
                .iter()
                .map(|game| {
                    let max =
                        game.rounds.iter().fold(Round::default(), Round::max);
                    max.red * max.green * max.blue
                })
                .sum::<usize>()
        },
    )
}
