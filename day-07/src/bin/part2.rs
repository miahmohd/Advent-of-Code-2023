use std::{cmp::Ordering, collections::HashMap};

fn main() {
    // let res = part1(include_str!("./input"));
    // println!("Part 1: {res}");

    // println!("########################################################");

    let res = part2(include_str!("../input"));
    println!("Part 2: {res}");
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd)]
enum CType {
    A,
    K,
    Q,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
    J,
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

        // println!("Hand: {:?}", self);
        // println!("Map: {:?}", map);
        // println!("Counts: {:?}", ordered_counts);

        let j_count: i32 = *map.get(&CType::J).unwrap_or(&0);

        let score = match ordered_counts[..] {
            [5] => 7,
            [4, 1] => match j_count {
                4 => 7,
                1 => 7,
                _ => 6,
            },
            [3, 2] => match j_count {
                3 => 7,
                2 => 7,
                _ => 5,
            },
            [3, 1, 1] => match j_count {
                3 => 6,
                1 => 6,
                _ => 4,
            },
            [2, 2, 1] => match j_count {
                2 => 6,
                1 => 5,
                _ => 3,
            },
            [2, 1, 1, 1] => match j_count {
                2 => 4,
                1 => 4,
                _ => 2,
            },
            [1, 1, 1, 1, 1] => match j_count {
                1 => 2,
                _ => 1,
            },

            _ => unreachable!(),
        };

        // println!("Score: {}", score);

        println!();

        score
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
        println!(
            "Hand compare: {:?} {:?} -> {:?}",
            self.cards,
            other.cards,
            self.score().partial_cmp(&other.score())
        );
        match self.score().partial_cmp(&other.score()) {
            Some(core::cmp::Ordering::Equal) => {
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    match b.partial_cmp(a) {
                        Some(Ordering::Equal) => {
                            println!("\tComparing {:?} {:?} = {:?}", b, a, Ordering::Equal);
                            continue;
                        }
                        ord => {
                            println!("\tComparing {:?} {:?} = {:?}", b, a, ord);
                            return ord;
                        }
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

fn part2(s: &str) -> i32 {
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

    for h in &hands {
        let _ = h.score();
    }

    hands.sort();

    // dbg!(&hands);

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + h.bid * (i as i32 + 1))
}

#[cfg(test)]
mod tests {
    use crate::{part2, CType};

    #[test]
    fn it_works_part2() {
        let result = part2(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, 22);
        // assert_eq!(CType::J < CType::C2, true);
    }

    //     #[test]
    //     fn it_works_part2_2() {
    //         let result = part2(
    //             "JQQQQ 1
    // JAKKK 10",
    //         );
    //         // assert_eq!(result, 12);
    //         assert_eq!(result, 21);
    //         // assert_eq!(CType::J < CType::C2, true);
    //     }
}
