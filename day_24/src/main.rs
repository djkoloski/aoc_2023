use std::{
    ops::{Add, Neg},
    str::FromStr,
};

use common::{solve, Context, Lines};

#[derive(Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

struct Hailstone {
    p: Vec3,
    v: Vec3,
}

impl FromStr for Hailstone {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s
            .split_once(" @ ")
            .context("expected position and velocity")?;
        let mut p_pieces = p.split(", ").map(str::trim).map(str::parse);
        let mut v_pieces = v.split(", ").map(str::trim).map(str::parse);

        Ok(Self {
            p: Vec3 {
                x: p_pieces.next().context("expected X position")??,
                y: p_pieces.next().context("expected Y position")??,
                z: p_pieces.next().context("expected Z position")??,
            },
            v: Vec3 {
                x: v_pieces.next().context("expected X velocity")??,
                y: v_pieces.next().context("expected Y velocity")??,
                z: v_pieces.next().context("expected Z velocity")??,
            },
        })
    }
}

impl Hailstone {
    fn intersection_2d(&self, other: &Self) -> Option<(f64, f64)> {
        let k = other.v.y - self.v.y * other.v.x / self.v.x;
        let t1 = (self.p.y + (other.p.x - self.p.x) * self.v.y / self.v.x
            - other.p.y)
            / k;
        let t0 = (other.v.x * t1 + (other.p.x - self.p.x)) / self.v.x;

        if t0 >= 0.0 && t1 >= 0.0 {
            Some((other.p.x + other.v.x * t1, other.p.y + other.v.y * t1))
        } else {
            None
        }
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<Hailstone>| {
            const MIN: f64 = 200000000000000.0;
            const MAX: f64 = 400000000000000.0;

            let mut total = 0;
            for i in 0..input.lines.len() {
                for j in i + 1..input.lines.len() {
                    if let Some((x, y)) =
                        input.lines[i].intersection_2d(&input.lines[j])
                    {
                        if (MIN..=MAX).contains(&x) && (MIN..=MAX).contains(&y)
                        {
                            total += 1;
                        }
                    }
                }
            }

            total
        },
        |input| {
            // This impl taken from a reddit comment. I did not enjoy this one.
            use nalgebra::{Matrix3, Matrix6, Vector6};

            let s0 = 0;
            let s1 = 1;
            let s2 = 2;

            let r0 = -input.lines[s0].p.cross(input.lines[s0].v)
                + input.lines[s1].p.cross(input.lines[s1].v);
            let r1 = -input.lines[s0].p.cross(input.lines[s0].v)
                + input.lines[s2].p.cross(input.lines[s2].v);
            let rhs = Vector6::new(r0.x, r0.y, r0.z, r1.x, r1.y, r1.z);

            fn cross_matrix(v: Vec3) -> Matrix3<f64> {
                Matrix3::new(0.0, -v.z, v.y, v.z, 0.0, -v.x, -v.y, v.x, 0.0)
            }

            let m0 = cross_matrix(input.lines[s0].v)
                - cross_matrix(input.lines[s1].v);
            let m1 = cross_matrix(input.lines[s0].v)
                - cross_matrix(input.lines[s2].v);
            let m2 = -cross_matrix(input.lines[s0].p)
                + cross_matrix(input.lines[s1].p);
            let m3 = -cross_matrix(input.lines[s0].p)
                + cross_matrix(input.lines[s2].p);
            let m = Matrix6::new(
                m0.m11, m0.m12, m0.m13, m2.m11, m2.m12, m2.m13, m0.m21, m0.m22,
                m0.m23, m2.m21, m2.m22, m2.m23, m0.m31, m0.m32, m0.m33, m2.m31,
                m2.m32, m2.m33, m1.m11, m1.m12, m1.m13, m3.m11, m3.m12, m3.m13,
                m1.m21, m1.m22, m1.m23, m3.m21, m3.m22, m3.m23, m1.m31, m1.m32,
                m1.m33, m3.m31, m3.m32, m3.m33,
            );
            let result = m.try_inverse().unwrap() * rhs;

            let x = result[0].round() as isize;
            let y = result[1].round() as isize;
            let z = result[2].round() as isize;

            x + y + z
        },
    )
}
