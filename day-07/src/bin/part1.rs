use std::{cmp::Ordering, collections::HashMap, ops::Add};

fn main() {
    let res = part1(include_str!("../input"));
    println!("Part 1: {res}");

    println!("########################################################");

    // let res = part2(include_str!("./input"));
    // println!("Part 2: {res}");
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd)]
enum CType {
    A,
    K,
    Q,
    J,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
}

impl From<char> for CType {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::C9,
            '8' => Self::C8,
            '7' => Self::C7,
            '6' => Self::C6,
            '5' => Self::C5,
            '4' => Self::C4,
            '3' => Self::C3,
            '2' => Self::C2,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<CType>,
    bid: i32,
}

impl Hand {
    fn score(&self) -> i32 {
        let mut map: HashMap<&CType, i32> = HashMap::new();

        for c in &self.cards {
            map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut ordered_counts: Vec<&i32> = map.values().collect();
        ordered_counts.sort_by(|a, b| b.cmp(a));

        println!("Hand: {:?}", self);
        println!("Map: {:?}", map);
        println!("Counts: {:?}", ordered_counts);

        match ordered_counts[..] {
            [5] => 7,
            [4, 1] => 6,
            [3, 2] => 5,
            [3, 1, 1] => 4,
            [2, 2, 1] => 3,
            [2, 1, 1, 1] => 2,
            [1, 1, 1, 1, 1] => 1,

            _ => unreachable!(),
        }

        // 0
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.score() == other.score()
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.score().partial_cmp(&other.score()) {
            Some(core::cmp::Ordering::Equal) => {
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    match b.partial_cmp(a) {
                        Some(Ordering::Equal) => continue,
                        ord => return ord,
                    }
                }
                None
            }
            ord => return ord,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            _ => Ordering::Equal,
        }
    }
}

fn part1(s: &str) -> i32 {
    let mut hands: Vec<Hand> = s
        .lines()
        .filter_map(|l| {
            if let Some((cards, bid)) = l.split_once(' ') {
                println!("Parsing {} {}", cards, bid);
                let cards: Vec<CType> = cards.chars().map(|c| c.into()).collect();
                let bid: i32 = bid.parse().expect("Should be i32");

                return Some(Hand { cards, bid });
            }
            None
        })
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + h.bid * (i as i32 + 1))
}

#[cfg(test)]
mod tests {
    use crate::{part1, CType};

    #[test]
    fn it_works_part1() {
        let result = part1(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, 6440);
    }
}
