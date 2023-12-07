use std::str::FromStr;

use common::{bail, solve, Context, Error, Lines};

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandKind {
    AllJokers,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    fn of(cards: [u8; 5]) -> Self {
        let mut counts = [0; 15];
        cards.iter().for_each(|&c| counts[c as usize] += 1);
        let counts = &counts[2..];
        match counts.iter().max().unwrap() {
            0 => Self::AllJokers,
            1 => Self::HighCard,
            2 => {
                if counts.iter().filter(|x| **x == 2).count() == 2 {
                    Self::TwoPair
                } else {
                    Self::OnePair
                }
            }
            3 => {
                if counts.iter().any(|x| *x == 2) {
                    Self::FullHouse
                } else {
                    Self::ThreeOfAKind
                }
            }
            4 => Self::FourOfAKind,
            5 => Self::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
}

impl Hand {
    fn new(cards: [u8; 5]) -> Self {
        Self { cards }
    }

    fn jokerify(self) -> Self {
        let mut cards = self.cards;
        cards.iter_mut().for_each(|c| {
            if *c == 11 {
                *c = 1;
            }
        });
        Self { cards }
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [0; 5];
        for (i, c) in s.chars().enumerate() {
            cards[i] = match c {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => bail!("invalid card '{c}'"),
            }
        }
        Ok(Self::new(cards))
    }
}

#[derive(Clone)]
struct HandAndBid {
    hand: Hand,
    bid: usize,
}

impl FromStr for HandAndBid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').context("expected hand and bid")?;
        Ok(Self {
            hand: Hand::from_str(hand)?,
            bid: bid.parse().context("invalid bid")?,
        })
    }
}

fn main() -> common::Result<()> {
    solve(
        |input: &Lines<HandAndBid>| {
            let mut sorted = input.lines.clone();
            sorted.sort_by_cached_key(|hand_and_bid| {
                (HandKind::of(hand_and_bid.hand.cards), hand_and_bid.hand)
            });
            sorted
                .iter()
                .enumerate()
                .map(|(i, hand_and_bid)| (i + 1) * hand_and_bid.bid)
                .sum::<usize>()
        },
        |input| {
            let mut sorted = input.lines.clone();
            sorted.iter_mut().for_each(|hand_and_bid| {
                hand_and_bid.hand = hand_and_bid.hand.jokerify()
            });
            sorted.sort_by_cached_key(|hand_and_bid| {
                let kind = HandKind::of(hand_and_bid.hand.cards);
                let jokers =
                    hand_and_bid.hand.cards.iter().filter(|c| **c == 1).count();
                let kind = match (kind, jokers) {
                    (kind, 0) => kind,
                    (HandKind::AllJokers, 5) => HandKind::FiveOfAKind,
                    (HandKind::HighCard, 1) => HandKind::OnePair,
                    (HandKind::HighCard, 2) | (HandKind::OnePair, 1) => {
                        HandKind::ThreeOfAKind
                    }
                    (HandKind::TwoPair, 1) => HandKind::FullHouse,
                    (HandKind::HighCard, 3)
                    | (HandKind::OnePair, 2)
                    | (HandKind::ThreeOfAKind, 1) => HandKind::FourOfAKind,
                    (HandKind::HighCard, 4)
                    | (HandKind::OnePair, 3)
                    | (HandKind::ThreeOfAKind, 2)
                    | (HandKind::FourOfAKind, 1) => HandKind::FiveOfAKind,
                    (kind, jokers) => {
                        unreachable!("kind: {kind:?}, jokers: {jokers}")
                    }
                };
                (kind, hand_and_bid.hand)
            });
            sorted
                .iter()
                .enumerate()
                .map(|(i, hand_and_bid)| (i + 1) * hand_and_bid.bid)
                .sum::<usize>()
        },
    )
}
