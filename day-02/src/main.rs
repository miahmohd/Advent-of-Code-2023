fn main() {
    let res = part1(include_str!("./input1"));
    println!("Part 1: {res}");

    let res = part2(include_str!("./input2"));
    println!("Part 2: {res}");
}

fn part1(s: &str) -> i32 {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let bag_content = (12, 13, 14);

    let res = s
        .lines()
        .filter_map(|l| {
            let (game, rest) = l.split_once(":").unwrap();
            let id: i32 = game
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .expect("Always a number");

            for subsets in rest.split(';') {
                let subsets = subsets.trim();

                for set in subsets.split(", ") {
                    let (n, color) = set.split_once(" ").unwrap();
                    let n: i32 = n.parse().unwrap();

                    match color {
                        "red" if n > bag_content.0 => return None,
                        "green" if n > bag_content.1 => return None,
                        "blue" if n > bag_content.2 => return None,
                        _ => continue,
                    }
                }
            }

            Some(id)
        })
        .sum();

    return res;
}

fn part2(s: &str) -> i32 {
    let res = s
        .lines()
        .map(|l| {
            let (_, rest) = l.split_once(":").unwrap();

            let mut min_bag_content = (0, 0, 0);

            for subsets in rest.split(';') {
                let subsets = subsets.trim();

                for set in subsets.split(", ") {
                    let (n, color) = set.split_once(" ").unwrap();
                    let n: i32 = n.parse().unwrap();

                    match color {
                        "red" => min_bag_content.0 = std::cmp::max(n, min_bag_content.0),
                        "green" => min_bag_content.1 = std::cmp::max(n, min_bag_content.1),
                        "blue" => min_bag_content.2 = std::cmp::max(n, min_bag_content.2),
                        _ => unreachable!(),
                    }
                }
            }

            min_bag_content.0 * min_bag_content.1 * min_bag_content.2
        })
        .sum();

    return res;
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works_part1() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8);
    }

    #[test]
    fn it_works_part2() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}
