use std::io::prelude::BufRead;

use common::{solve, Context, Error, Input};

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn ways_to_win(&self) -> usize {
        let min_hold = ((self.time as f64
            - (self.time as f64 * self.time as f64
                - 4.0 * (self.distance + 1) as f64)
                .sqrt())
            / 2.0)
            .ceil() as usize;
        self.time + 1 - min_hold * 2
    }
}

struct Races {
    races: Vec<Race>,
}

impl Input for Races {
    fn parse_reader<R: BufRead>(reader: R) -> common::Result<Self> {
        let mut lines = reader.lines();
        let times_line = lines.next().context("expected times")??;
        let times = times_line
            .split(' ')
            .filter(|s| !s.is_empty())
            .skip(1)
            .map(str::parse);
        let distances_line = lines.next().context("expected distances")??;
        let distances = distances_line
            .split(' ')
            .filter(|s| !s.is_empty())
            .skip(1)
            .map(str::parse);

        Ok(Self {
            races: times
                .zip(distances)
                .map(|(time, distance)| {
                    Ok(Race {
                        time: time?,
                        distance: distance?,
                    })
                })
                .collect::<Result<_, Error>>()?,
        })
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Races| {
            input.races.iter().map(Race::ways_to_win).product::<usize>()
        },
        |input| {
            Race {
                time: input.races.iter().fold(0, |time, race| {
                    time * 10_usize.pow(race.time.ilog10() + 1) + race.time
                }),
                distance: input.races.iter().fold(0, |distance, race| {
                    distance * 10_usize.pow(race.distance.ilog10() + 1)
                        + race.distance
                }),
            }
            .ways_to_win()
        },
    )
}
