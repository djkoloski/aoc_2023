use std::{collections::HashMap, io::prelude::BufRead, str::FromStr};

use common::{solve, Context, Input};

#[derive(Debug)]
struct Run {
    start: usize,
    len: usize,
}

#[derive(Debug)]
struct Range {
    src_start: usize,
    dest_start: usize,
    len: usize,
}

impl Range {
    fn map(&self, input: usize) -> Option<usize> {
        if input >= self.src_start && input < self.src_start + self.len {
            Some(input - self.src_start + self.dest_start)
        } else {
            None
        }
    }

    fn map_run(
        &self,
        run: Run,
        unmapped: &mut Vec<Run>,
        mapped: &mut Vec<Run>,
    ) {
        let map_end = self.src_start + self.len;
        let run_end = run.start + run.len;

        if self.src_start <= run.start && run.start < map_end {
            // run interrupts map
            mapped.push(Run {
                start: run.start - self.src_start + self.dest_start,
                len: usize::min(run.len, map_end - run.start),
            });
            if run_end > map_end {
                unmapped.push(Run {
                    start: map_end,
                    len: run_end - map_end,
                });
            }
        } else if run.start <= self.src_start && self.src_start < run_end {
            // map interrupts run
            if run.start < self.src_start {
                unmapped.push(Run {
                    start: run.start,
                    len: self.src_start - run.start,
                });
            }
            mapped.push(Run {
                start: self.dest_start,
                len: usize::min(self.len, run_end - self.src_start),
            });
            if map_end < run_end {
                unmapped.push(Run {
                    start: map_end,
                    len: run_end - map_end,
                });
            }
        } else {
            unmapped.push(run);
        }
    }
}

impl FromStr for Range {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(' ').map(str::parse);
        let dest = pieces.next().context("expected destination")??;
        let src = pieces.next().context("expected source")??;
        let len = pieces.next().context("expected length")??;
        Ok(Self {
            src_start: src,
            dest_start: dest,
            len,
        })
    }
}

struct Map {
    dest: String,
    ranges: Vec<Range>,
}

impl Map {
    fn map(&self, value: usize) -> usize {
        self.ranges
            .iter()
            .filter_map(|r| r.map(value))
            .next()
            .unwrap_or(value)
    }

    fn map_run(&self, run: Run, mapped: &mut Vec<Run>) {
        let mut unmapped = vec![run];

        for range in self.ranges.iter() {
            let mut new_unmapped = Vec::new();
            for run in unmapped.drain(..) {
                range.map_run(run, &mut new_unmapped, mapped);
            }
            core::mem::swap(&mut unmapped, &mut new_unmapped);
        }

        mapped.extend(unmapped);
    }
}

struct Almanac {
    seeds: Vec<usize>,
    maps: HashMap<String, Map>,
}

impl Input for Almanac {
    fn parse_reader<R: BufRead>(reader: R) -> common::Result<Self> {
        let mut lines = reader.lines();

        let seeds_line = lines.next().context("expected seeds")??;
        let seeds = seeds_line
            .strip_prefix("seeds: ")
            .context("expected seeds line to start with 'seeds: '")?
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        assert_eq!(lines.next().context("expected newline")??, "");

        let mut maps = HashMap::new();
        while let Some(map_line) = lines.next() {
            let map_line = map_line?;
            let (source, dest) = map_line
                .strip_suffix(" map:")
                .context("expected map line to end with ' map:'")?
                .split_once("-to-")
                .context("expected '-to-' separator")?;
            let mut ranges = Vec::new();

            for range_line in lines.by_ref() {
                let range_line = range_line?;

                if range_line.is_empty() {
                    break;
                }

                ranges.push(range_line.parse()?);
            }

            maps.insert(
                source.to_string(),
                Map {
                    dest: dest.to_string(),
                    ranges,
                },
            );
        }

        Ok(Self { seeds, maps })
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Almanac| {
            let mut values = input.seeds.clone();
            let mut name = "seed";

            while name != "location" {
                let map = input.maps.get(name).unwrap();
                values.iter_mut().for_each(|value| *value = map.map(*value));

                name = &map.dest;
            }

            *values.iter().min().unwrap()
        },
        |input| {
            let mut runs = input
                .seeds
                .iter()
                .step_by(2)
                .zip(input.seeds.iter().skip(1).step_by(2))
                .map(|(&start, &len)| Run { start, len })
                .collect::<Vec<_>>();
            let mut name = "seed";

            while name != "location" {
                let map = input.maps.get(name).unwrap();
                let mut new_runs = Vec::new();
                runs.drain(..)
                    .for_each(|run| map.map_run(run, &mut new_runs));
                core::mem::swap(&mut runs, &mut new_runs);

                name = &map.dest;
            }

            runs.iter().min_by_key(|x| x.start).unwrap().start
        },
    )
}
