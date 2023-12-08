use std::collections::{BTreeMap, HashSet};

fn main() {
    let res = part1(include_str!("./input"));
    println!("Part 1: {res}");

    println!("########################################################");

    // let res = part2(include_str!("./input"));
    // println!("Part 2: {res}");
}

fn part1(s: &str) -> u64 {
    let lines: Vec<&str> = s.lines().collect();
    let insts: Vec<char> = lines[0].chars().collect();
    let mut map: BTreeMap<&str, (&str, &str)> = BTreeMap::new();

    for l in lines.iter().skip(2) {
        map.insert(&l[0..3], (&l[7..10], &l[12..15]));
    }

    dbg!(&insts);
    dbg!(&map);

    let current_nodes: Vec<&str> = map
        .keys()
        .filter(|k| k.chars().nth_back(0) == Some('A'))
        .map(|k| *k)
        .collect();
    let mut last_node_steps = Vec::with_capacity(current_nodes.len());

    for node in current_nodes {
        let mut current = node;
        let mut steps = 0;
        let mut visited_set = HashSet::new();
        let mut step_until_last = Vec::new();
        visited_set.insert((current, steps % insts.len()));

        loop {
            let inst = insts[steps % insts.len()];

            let next = map.get(current).unwrap();

            match inst {
                'R' => current = next.1,
                'L' => current = next.0,
                _ => unreachable!(),
            }

            steps += 1;

            // println!(
            //     "Node: {}, Current: {}, Steps: {}, inst: {}",
            //     node,
            //     current,
            //     steps,
            //     steps % insts.len()
            // );

            if current.chars().nth_back(0) == Some('Z') {
                step_until_last.push((node, current, steps));
            }

            if visited_set.contains(&(current, steps % insts.len())) {
                break;
            }

            visited_set.insert((current, steps % insts.len()));
        }
        // println!();
        last_node_steps.push(step_until_last);
    }

    println!("Steps until last: {:?}", last_node_steps);

    let mut acc = 1;

    for l in last_node_steps {
        let x = l.first().unwrap().2 as u64;
        acc = num::integer::lcm(acc, x);
    }

    acc
}

#[cfg(test)]
mod tests {
    use crate::part1;

    // #[test]
    //     fn it_works_part1() {
    //         assert_eq!(
    //             part1(
    //                 "RL

    // AAA = (BBB, CCC)
    // BBB = (DDD, EEE)
    // CCC = (ZZZ, GGG)
    // DDD = (DDD, DDD)
    // EEE = (EEE, EEE)
    // GGG = (GGG, GGG)
    // ZZZ = (ZZZ, ZZZ)"
    //             ),
    //             2
    //         );

    //         assert_eq!(
    //             part1(
    //                 "LLR

    // AAA = (BBB, BBB)
    // BBB = (AAA, ZZZ)
    // ZZZ = (ZZZ, ZZZ)"
    //             ),
    //             6
    //         );
    //     }

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
            7
        );
    }
}
