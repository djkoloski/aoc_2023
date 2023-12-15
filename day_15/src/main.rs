use std::str::FromStr;

use common::{solve, Context, List};

fn hash(s: &str) -> usize {
    s.chars().fold(0, |a, c| (a + c as usize) * 17 % 256)
}

struct Instruction {
    label: String,
    op: Op,
}

impl FromStr for Instruction {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, op) =
            s.split_at(s.find(['=', '-']).context("invalid instruction")?);
        Ok(Self {
            label: label.to_string(),
            op: op.parse()?,
        })
    }
}

enum Op {
    Remove,
    Insert(usize),
}

impl FromStr for Op {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(focal_length) = s.strip_prefix('=') {
            Ok(Self::Insert(focal_length.parse()?))
        } else {
            Ok(Self::Remove)
        }
    }
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

fn main() -> common::Result<()> {
    solve(
        |input: &List<String>| {
            input.elements.iter().map(|x| hash(x)).sum::<usize>()
        },
        |input| {
            let mut buckets = vec![Vec::<Lens>::new(); 256];
            for i in input.elements.iter() {
                let instruction = i.parse::<Instruction>().unwrap();
                let index = hash(&instruction.label);
                match instruction.op {
                    Op::Remove => {
                        if let Some(l) = buckets[index]
                            .iter()
                            .position(|lens| lens.label == instruction.label)
                        {
                            buckets[index].remove(l);
                        }
                    }
                    Op::Insert(focal_length) => {
                        if let Some(lens) = buckets[index]
                            .iter_mut()
                            .find(|lens| lens.label == instruction.label)
                        {
                            lens.focal_length = focal_length;
                        } else {
                            buckets[index].push(Lens {
                                label: instruction.label,
                                focal_length,
                            });
                        }
                    }
                }
            }
            // println!("{buckets:#?}");
            buckets
                .iter()
                .enumerate()
                .map(|(b, bucket)| {
                    bucket
                        .iter()
                        .enumerate()
                        .map(|(l, lens)| (1 + b) * (1 + l) * lens.focal_length)
                        .sum::<usize>()
                })
                .sum::<usize>()
        },
    )
}
