// NOTE: what a horrible problem. the general case is completely intractable,
// it's only solvable if you guess what it's "actually" doing under the hood.
// this already happened on day 8 this year, and i'm really disappointed that it
// happened again.

use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use common::{bail, solve, Context, Lines};

#[derive(PartialEq, Eq)]
enum Kind {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

struct Module {
    kind: Kind,
    name: String,
    outputs: Vec<String>,
}

impl FromStr for Module {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, outputs) =
            s.split_once(" -> ").context("expected arrow")?;
        let kind = match &prefix[0..1] {
            "b" => Kind::Broadcaster,
            "%" => Kind::FlipFlop,
            "&" => Kind::Conjunction,
            _ => bail!("unrecognized module kind"),
        };

        Ok(Self {
            kind,
            name: prefix
                .strip_prefix(['%', '&'])
                .unwrap_or("broadcaster")
                .to_string(),
            outputs: outputs.split(", ").map(str::to_string).collect(),
        })
    }
}

#[allow(clippy::type_complexity)]
fn setup(
    input: &[Module],
) -> (
    HashMap<String, &Module>,
    HashMap<String, Vec<String>>,
    HashMap<String, bool>,
) {
    let modules = input
        .iter()
        .map(|module| (module.name.clone(), module))
        .collect::<HashMap<_, _>>();

    let mut inputs = HashMap::new();
    let mut state = HashMap::new();

    for module in input.iter() {
        state.insert(module.name.clone(), false);
        for output in module.outputs.iter() {
            inputs
                .entry(output.clone())
                .or_insert(Vec::new())
                .push(module.name.clone());
        }
    }

    (modules, inputs, state)
}

#[allow(clippy::too_many_arguments)]
fn step_one<'a>(
    modules: &'a HashMap<String, &Module>,
    inputs: &HashMap<String, Vec<String>>,
    state: &mut HashMap<String, bool>,
    pending: &mut VecDeque<(&'a str, bool)>,
    total_high: &mut usize,
    total_low: &mut usize,
    target: &str,
    value: bool,
) {
    if value {
        *total_high += 1;
    } else {
        *total_low += 1;
    }

    if !modules.contains_key(target) {
        return;
    }

    let module = &modules[target];
    match module.kind {
        Kind::Broadcaster => {
            for output in module.outputs.iter() {
                pending.push_back((output, value));
            }
        }
        Kind::FlipFlop => {
            if !value {
                let s = state.get_mut(target).unwrap();
                *s = !*s;
                for output in module.outputs.iter() {
                    pending.push_back((output, *s));
                }
            }
        }
        Kind::Conjunction => {
            *state.get_mut(target).unwrap() =
                !inputs[target].iter().all(|i| state[i.as_str()]);
            for output in module.outputs.iter() {
                pending.push_back((output, state[target]));
            }
        }
    }
}

fn step(
    modules: &HashMap<String, &Module>,
    inputs: &HashMap<String, Vec<String>>,
    state: &mut HashMap<String, bool>,
) -> (usize, usize) {
    let mut pending = VecDeque::new();
    let mut total_low = 0;
    let mut total_high = 0;

    pending.push_back(("broadcaster", false));
    while let Some((target, value)) = pending.pop_front() {
        step_one(
            modules,
            inputs,
            state,
            &mut pending,
            &mut total_high,
            &mut total_low,
            target,
            value,
        );
    }

    (total_low, total_high)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<Module>| {
            let (modules, inputs, mut state) = setup(&input.lines);

            let mut total_low = 0;
            let mut total_high = 0;
            for _ in 0..1000 {
                let (low, high) = step(&modules, &inputs, &mut state);
                total_low += low;
                total_high += high;
            }

            total_low * total_high
        },
        |input: &Lines<Module>| {
            let (modules, inputs, mut state) = setup(&input.lines);

            let factors = &inputs[&inputs["rx"][0]];

            println!("{factors:?}");

            let mut loop_lengths = vec![None; factors.len()];
            for i in 1.. {
                let mut pending = VecDeque::new();
                let mut total_low = 0;
                let mut total_high = 0;

                pending.push_back(("broadcaster", false));
                while let Some((target, value)) = pending.pop_front() {
                    step_one(
                        &modules,
                        &inputs,
                        &mut state,
                        &mut pending,
                        &mut total_high,
                        &mut total_low,
                        target,
                        value,
                    );
                    for (f, factor) in factors.iter().enumerate() {
                        if state[factor] {
                            loop_lengths[f] = Some(i);
                        }
                    }
                }

                if loop_lengths.iter().all(Option::is_some) {
                    break;
                }
            }

            let mut result = 1;
            for l in loop_lengths.iter() {
                result = lcm(result, l.unwrap());
            }

            result
        },
    )
}
