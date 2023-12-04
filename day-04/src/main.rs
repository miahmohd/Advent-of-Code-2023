use std::collections::HashSet;

fn main() {
    let res = part1(include_str!("./input1"));
    println!("Part 1: {res}");

    let res = part2(include_str!("./input2"));
    println!("Part 2: {res}");
}

fn part1(s: &str) -> i32 {
    s.lines()
        .map(|l| {
            let (_, numbers) = l.split_once(':').unwrap();
            let (win_n, my_n) = numbers.split_once('|').unwrap();
            let win_n: HashSet<i32> = win_n
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse().expect("Should be i32"))
                .collect();

            let my_n: HashSet<i32> = my_n
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse().expect("Should be i32"))
                .collect();

            let matches = win_n.intersection(&my_n).count();

            // println!("Matches: {}", matches);

            2_i32.pow(matches as u32) / 2
            //
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Card {
    n: usize,
    copies: i32,
    matches: usize,
}

fn part2(s: &str) -> i32 {
    // Parse cards in BTreeMap
    let mut cards: Vec<Card> = s
        .lines()
        .map(|l| {
            let (card, numbers) = l.split_once(':').unwrap();

            let card_n = card
                .split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .parse()
                .expect("Should be number");

            let (win_n, my_n) = numbers.split_once('|').unwrap();
            let win_n: HashSet<i32> = win_n
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse().expect("Should be i32"))
                .collect();

            let my_n: HashSet<i32> = my_n
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse().expect("Should be i32"))
                .collect();

            let matches = win_n.intersection(&my_n).count();

            Card {
                n: card_n,
                matches: matches,
                copies: 1,
            }
            //
        })
        .collect();

    for i in 0..cards.len() {
        let curr = cards[i];

        for _ in 0..curr.copies {
            for j in 0..curr.matches {
                // println!("card {} inc {}", curr.n, i + 1 + j + 1);
                cards[curr.n + j].copies += 1;
            }
        }
    }

    cards.iter().map(|c| c.copies).sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works_part1() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13);
    }

    #[test]
    fn it_works_part2() {
        let result = part2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 30);
    }
}
