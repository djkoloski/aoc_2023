use common::{solve, Lines};

const NAMES: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<String>| {
            input
                .lines
                .iter()
                .map(|line| {
                    let mut digits = line
                        .chars()
                        .filter(char::is_ascii_digit)
                        .map(|c| c.to_digit(10).unwrap());
                    let first = digits.next().unwrap();
                    let last = digits.last().unwrap_or(first);
                    first * 10 + last
                })
                .sum::<u32>()
        },
        |input| {
            input
                .lines
                .iter()
                .map(|line| {
                    let mut digits = line
                        .chars()
                        .enumerate()
                        .filter(|(_, c)| c.is_ascii_digit())
                        .map(|(i, c)| (i, c.to_digit(10).unwrap()));
                    let mut first = digits.next().unwrap();
                    let mut last = digits.last().unwrap_or(first);

                    for (i, name) in NAMES.iter().enumerate() {
                        if let Some(index) = line.find(name) {
                            if index < first.0 {
                                first.0 = index;
                                first.1 = i as u32 + 1;
                            }
                        }
                        if let Some(index) = line.rfind(name) {
                            if index > last.0 {
                                last.0 = index;
                                last.1 = i as u32 + 1;
                            }
                        }
                    }

                    first.1 * 10 + last.1
                })
                .sum::<u32>()
        },
    )
}
