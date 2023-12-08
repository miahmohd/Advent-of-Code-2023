use std::{collections::HashMap, ops::Deref};

fn main() {
    let res = part1(include_str!("./input"));
    println!("Part 1: {res}");

    println!("########################################################");

    // let res = part2(include_str!("./input"));
    // println!("Part 2: {res}");
}

fn part1(s: &str) -> i32 {
    let lines: Vec<&str> = s.lines().collect();
    let insts: Vec<char> = lines[0].chars().collect();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for l in lines.iter().skip(2) {
        map.insert(&l[0..3], (&l[7..10], &l[12..15]));
    }

    dbg!(&insts);
    dbg!(&map);

    let mut steps = 0;
    let mut current_nodes: Vec<&str> = map
        .keys()
        .filter(|k| k.chars().nth_back(0) == Some('A'))
        .map(|k| *k)
        .collect();
    let mut next_nodes = Vec::with_capacity(current_nodes.len());

    loop {
        let inst = insts[steps % insts.len()];

        for current in &current_nodes {
            let next = map.get(current).unwrap();

            match inst {
                'R' => next_nodes.push(next.1),
                'L' => next_nodes.push(next.0),
                _ => unreachable!(),
            }
        }

        steps += 1;

        if steps % 1_000 == 0 {
            println!("Step: {}", steps);
            println!("Curr nodes: {:?}", current_nodes);
            println!("Next nodes: {:?}", next_nodes);
        }

        if next_nodes
            .iter()
            .all(|k| k.chars().nth_back(0) == Some('Z'))
        {
            break;
        }

        current_nodes.clear();
        current_nodes.append(&mut next_nodes);
    }

    steps as i32
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works_part1() {
        assert_eq!(
            part1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            2
        );

        assert_eq!(
            part1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        );
    }

    #[test]
    fn it_works_part2() {
        assert_eq!(
            part1(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }
}
