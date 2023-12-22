use std::str::FromStr;

use common::{solve, Context as _, Lines};

#[derive(Clone, Copy, Debug)]
struct Vec3 {
    v: [i32; 3],
}

impl FromStr for Vec3 {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(',').map(str::parse);
        Ok(Self {
            v: [
                pieces.next().context("expected x coordinate")??,
                pieces.next().context("expected y coordinate")??,
                pieces.next().context("expected z coordinate")??,
            ],
        })
    }
}

#[derive(Clone, Debug)]
struct Volume {
    lower: Vec3,
    upper: Vec3,
}

impl Volume {
    fn support_point(&self, other: &Self) -> Option<Vec3> {
        if self.lower.v[0] > other.upper.v[0]
            || self.upper.v[0] < other.lower.v[0]
            || self.lower.v[1] > other.upper.v[1]
            || self.upper.v[1] < other.lower.v[1]
        {
            return None;
        }

        Some(Vec3 {
            v: [
                i32::max(self.lower.v[0], other.lower.v[0]),
                i32::max(self.lower.v[1], other.lower.v[1]),
                i32::max(self.lower.v[2], self.upper.v[2]),
            ],
        })
    }
}

impl FromStr for Volume {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once('~')
            .context("expected start and end coordinates")?;
        Ok(Self {
            lower: start.parse()?,
            upper: end.parse()?,
        })
    }
}

fn settle(bricks: &mut Vec<Volume>) -> usize {
    let mut fell = 0;

    // Pre-bricks are sorted by lowest Z
    bricks.sort_unstable_by_key(|b| b.lower.v[2]);
    // Drop the lowest brick to the ground
    if bricks[0].lower.v[2] != 1 {
        fell += 1;
    }
    bricks[0].upper.v[2] -= bricks[0].lower.v[2] - 1;
    bricks[0].lower.v[2] = 1;
    for i in 1..bricks.len() {
        // Drop brick `i`
        let mut supports = Vec::new();
        let mut new_z = 1;

        for j in (0..i).rev() {
            // If `j` supports `i`, drop it and sort into post-bricks
            // post-bricks are sorted by highest Z
            if let Some(p) = bricks[j].support_point(&bricks[i]) {
                if p.v[2] + 1 > new_z {
                    supports.clear();
                    new_z = p.v[2] + 1;
                }
                if p.v[2] + 1 == new_z {
                    supports.push(j);
                }
            }
        }

        let d = bricks[i].lower.v[2] - new_z;
        if d > 0 {
            bricks[i].lower.v[2] -= d;
            bricks[i].upper.v[2] -= d;
            bricks[0..=i].sort_unstable_by_key(|b| b.upper.v[2]);

            fell += 1;
        }
    }

    fell
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<Volume>| {
            let mut bricks = input.lines.clone();
            settle(&mut bricks);

            let mut removable = 0;
            for i in 0..bricks.len() {
                let mut b = bricks.clone();
                b.swap_remove(i);
                if settle(&mut b) == 0 {
                    removable += 1;
                }
            }

            removable
        },
        |input| {
            let mut bricks = input.lines.clone();
            settle(&mut bricks);

            let mut settled = 0;
            for i in 0..bricks.len() {
                let mut b = bricks.clone();
                b.swap_remove(i);
                settled += settle(&mut b);
            }

            settled
        },
    )
}
