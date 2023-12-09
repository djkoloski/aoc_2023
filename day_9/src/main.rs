use std::str::FromStr;

use common::{solve, Lines};

struct History {
    values: Vec<isize>,
}

impl History {
    fn differences(&self) -> Vec<Vec<isize>> {
        let mut differences = Vec::new();
        loop {
            let mut difference = Vec::new();
            let base = differences.last().unwrap_or(&self.values);

            let mut all_zero = true;
            for window in base.windows(2) {
                let d = window[1] - window[0];
                all_zero = all_zero && d == 0;
                difference.push(d);
            }

            if all_zero {
                break;
            }
            differences.push(difference);
        }
        differences
    }

    fn predict(&self) -> isize {
        self.differences()
            .iter()
            .map(|d| d.last().unwrap())
            .sum::<isize>()
            + self.values.last().unwrap()
    }

    fn predict_rev(&self) -> isize {
        self.values.first().unwrap()
            - self
                .differences()
                .iter()
                .rev()
                .fold(0, |prev, d| d.first().unwrap() - prev)
    }
}

impl FromStr for History {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(History {
            values: s.split(' ').map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<History>| {
            input.lines.iter().map(|line| line.predict()).sum::<isize>()
        },
        |input: &Lines<History>| {
            input
                .lines
                .iter()
                .map(|line| line.predict_rev())
                .sum::<isize>()
        },
    )
}
