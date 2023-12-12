use std::{collections::HashMap, str::FromStr};

use common::{bail, solve, Context, Lines};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = common::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => bail!("unrecognized char '{value}'"),
        })
    }
}

struct Report {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl FromStr for Report {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, groups) =
            s.split_once(' ').context("expected springs and groups")?;
        Ok(Self {
            springs: springs
                .chars()
                .map(Spring::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            groups: groups
                .split(',')
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Report {
    fn quintuple(&self) -> Report {
        let mut springs = self.springs.clone();
        for _ in 0..4 {
            springs.push(Spring::Unknown);
            springs.extend(self.springs.iter().cloned());
        }

        Report {
            springs,
            groups: self.groups.repeat(5),
        }
    }

    fn all_solutions(&self) -> usize {
        let mut cache = HashMap::new();
        self.solutions(0, 0, 0, &mut cache)
    }

    #[inline]
    fn solutions(
        &self,
        s: usize,
        g: usize,
        r: usize,
        cache: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        if let Some(result) = cache.get(&(s, g, r)) {
            return *result;
        }

        let result = if s == self.springs.len() {
            if g == self.groups.len()
                || (g + 1 == self.groups.len() && self.groups[g] == r)
            {
                1
            } else {
                0
            }
        } else {
            match self.springs[s] {
                Spring::Operational => {
                    self.solutions_operational(s, g, r, cache)
                }
                Spring::Damaged => self.solutions_damaged(s, g, r, cache),
                Spring::Unknown => {
                    self.solutions_operational(s, g, r, cache)
                        + self.solutions_damaged(s, g, r, cache)
                }
            }
        };
        cache.insert((s, g, r), result);
        result
    }

    #[inline]
    fn solutions_operational(
        &self,
        s: usize,
        g: usize,
        r: usize,
        cache: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        if r == 0 {
            self.solutions(s + 1, g, 0, cache)
        } else if r != self.groups[g] {
            0
        } else {
            self.solutions(s + 1, g + 1, 0, cache)
        }
    }

    #[inline]
    fn solutions_damaged(
        &self,
        s: usize,
        g: usize,
        r: usize,
        cache: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        if g < self.groups.len() && r < self.groups[g] {
            self.solutions(s + 1, g, r + 1, cache)
        } else {
            0
        }
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<Report>| {
            input
                .lines
                .iter()
                .map(|report| report.all_solutions())
                .sum::<usize>()
        },
        |input| {
            input
                .lines
                .iter()
                .map(Report::quintuple)
                .map(|report| report.all_solutions())
                .sum::<usize>()
        },
    )
}
