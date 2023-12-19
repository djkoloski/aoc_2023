use std::{collections::HashMap, ops::Range, str::FromStr};

use common::{bail, solve, Context as _};

enum Operation {
    LessThan,
    GreaterThan,
}

impl Operation {
    fn compare(&self, lhs: usize, rhs: usize) -> bool {
        match self {
            Self::LessThan => lhs < rhs,
            Self::GreaterThan => lhs > rhs,
        }
    }

    // Splits range into (true, false)
    fn split(
        &self,
        range: Range<usize>,
        value: usize,
    ) -> (Range<usize>, Range<usize>) {
        match self {
            Self::LessThan => split_range(range, value),
            Self::GreaterThan => {
                let (f, t) = split_range(range, value + 1);
                (t, f)
            }
        }
    }
}

enum Field {
    X,
    M,
    A,
    S,
}

impl Field {
    fn pick(&self, part: &Part) -> usize {
        match self {
            Self::X => part.x,
            Self::M => part.m,
            Self::A => part.a,
            Self::S => part.s,
        }
    }
}

struct Condition {
    field: Field,
    operation: Operation,
    value: usize,
}

impl Condition {
    fn applies(&self, part: &Part) -> bool {
        self.operation.compare(self.field.pick(part), self.value)
    }

    // Splits parts into (true, false)
    fn split(&self, parts: AllParts) -> (AllParts, AllParts) {
        match self.field {
            Field::X => {
                let (t, f) = self.operation.split(parts.x.clone(), self.value);
                (
                    AllParts {
                        x: t,
                        ..parts.clone()
                    },
                    AllParts { x: f, ..parts },
                )
            }
            Field::M => {
                let (t, f) = self.operation.split(parts.m.clone(), self.value);
                (
                    AllParts {
                        m: t,
                        ..parts.clone()
                    },
                    AllParts { m: f, ..parts },
                )
            }
            Field::A => {
                let (t, f) = self.operation.split(parts.a.clone(), self.value);
                (
                    AllParts {
                        a: t,
                        ..parts.clone()
                    },
                    AllParts { a: f, ..parts },
                )
            }
            Field::S => {
                let (t, f) = self.operation.split(parts.s.clone(), self.value);
                (
                    AllParts {
                        s: t,
                        ..parts.clone()
                    },
                    AllParts { s: f, ..parts },
                )
            }
        }
    }
}

impl FromStr for Condition {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (field, rest) = s.split_at(1);
        let (operation, value) = rest.split_at(1);

        Ok(Self {
            field: match field {
                "x" => Field::X,
                "m" => Field::M,
                "a" => Field::A,
                "s" => Field::S,
                _ => bail!("unrecognized field {field}"),
            },
            operation: match operation {
                "<" => Operation::LessThan,
                ">" => Operation::GreaterThan,
                _ => bail!("unrecognized operation {operation}"),
            },
            value: value.parse()?,
        })
    }
}

enum Destination {
    Accept,
    Reject,
    Workflow(String),
}

impl FromStr for Destination {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Workflow(s.to_string()),
        })
    }
}

struct Rule {
    condition: Condition,
    destination: Destination,
}

impl FromStr for Rule {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, d) = s.split_once(':').context("expected colon")?;
        Ok(Self {
            condition: c.parse()?,
            destination: d.parse()?,
        })
    }
}

struct Workflow {
    rules: Vec<Rule>,
    default: Destination,
}

impl Workflow {
    fn destination(&self, part: &Part) -> &Destination {
        for rule in self.rules.iter() {
            if rule.condition.applies(part) {
                return &rule.destination;
            }
        }
        &self.default
    }
}

impl FromStr for Workflow {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rules, default) = s
            .rsplit_once(',')
            .context("expected trailing default destination")?;
        Ok(Self {
            rules: rules
                .split(',')
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
            default: default.parse()?,
        })
    }
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl FromStr for Part {
    type Err = common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s
            .strip_prefix('{')
            .context("expected leading {")?
            .strip_suffix('}')
            .context("expected trailing }")?
            .split(',')
            .map(|f| {
                Result::<_, common::Error>::Ok(
                    f.split_once('=')
                        .context("expected = to separate field and value")?
                        .1
                        .parse()?,
                )
            });

        Ok(Self {
            x: fields.next().context("expected an x field")??,
            m: fields.next().context("expected an m field")??,
            a: fields.next().context("expected an a field")??,
            s: fields.next().context("expected an s field")??,
        })
    }
}

struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl common::Input for Input {
    fn parse_reader<R: std::io::prelude::BufRead>(
        reader: R,
    ) -> common::Result<Self> {
        let mut workflows = HashMap::new();
        let mut parts = Vec::new();
        let mut lines = reader.lines();

        loop {
            let line = lines.next().context("expected workflows")??;

            if line.is_empty() {
                break;
            }

            let (name, rest) =
                line.split_once('{').context("expected name prefix")?;
            workflows.insert(
                name.to_string(),
                rest.strip_suffix('}')
                    .context("expected workflow to end with }")?
                    .parse()?,
            );
        }

        for line in lines {
            let line = line?;
            parts.push(line.parse()?);
        }

        Ok(Self { workflows, parts })
    }
}

fn split_range(range: Range<usize>, x: usize) -> (Range<usize>, Range<usize>) {
    (
        range.start..usize::max(range.start, x),
        usize::min(range.end, x)..range.end,
    )
}

#[derive(Clone)]
struct AllParts {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl AllParts {
    fn count(&self) -> usize {
        (self.x.end - self.x.start)
            * (self.m.end - self.m.start)
            * (self.a.end - self.a.start)
            * (self.s.end - self.s.start)
    }
}

fn count_accepted(
    workflows: &HashMap<String, Workflow>,
    w: &str,
    mut parts: AllParts,
) -> usize {
    let workflow = &workflows[w];
    let mut total = 0;
    for rule in workflow.rules.iter() {
        let (t, f) = rule.condition.split(parts);
        total += match &rule.destination {
            Destination::Accept => t.count(),
            Destination::Reject => 0,
            Destination::Workflow(w) => count_accepted(workflows, w, t),
        };
        parts = f;
    }
    total
        + match &workflow.default {
            Destination::Accept => parts.count(),
            Destination::Reject => 0,
            Destination::Workflow(w) => count_accepted(workflows, w, parts),
        }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Input| {
            input
                .parts
                .iter()
                .filter(|part| {
                    let mut workflow = "in";
                    loop {
                        match input.workflows[workflow].destination(part) {
                            Destination::Accept => break true,
                            Destination::Reject => break false,
                            Destination::Workflow(w) => workflow = w,
                        }
                    }
                })
                .map(|p| p.x + p.m + p.a + p.s)
                .sum::<usize>()
        },
        |input| {
            let start = AllParts {
                x: 1..4001,
                m: 1..4001,
                a: 1..4001,
                s: 1..4001,
            };
            count_accepted(&input.workflows, "in", start)
        },
    )
}
