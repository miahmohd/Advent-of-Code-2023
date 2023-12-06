use core::num;
use std::collections::VecDeque;

fn main() {
    let res = part1(include_str!("./input"));
    println!("Part 1: {res}");

    let res = part2(include_str!("./input"));
    println!("Part 2: {res}");
}

#[derive(Debug, Clone, Copy)]
struct MapRange {
    src: i64,
    dest: i64,
    len: i64,
}

fn parse_seeds(l: &str) -> Vec<i64> {
    let mut r = Vec::new();

    for s in l.split_ascii_whitespace() {
        match s.parse() {
            Ok(n) => r.push(n),
            Err(_) => continue,
        }
    }

    r
}

fn part1(s: &str) -> i64 {
    let lines: Vec<_> = s.lines().collect();

    // let blocks: Vec<&[&str]> = lines.split(|l| l.is_empty()).collect();
    let mut blocks = lines.split(|l| l.is_empty());

    //
    let seeds = parse_seeds(blocks.next().unwrap()[0]);

    let maps: Vec<Vec<MapRange>> = blocks
        .map(|b| {
            b.iter()
                .skip(1)
                .map(|m| {
                    let nums: Vec<i64> = m
                        .split_ascii_whitespace()
                        .map(|n| n.parse().expect("Should be i64"))
                        .collect();

                    MapRange {
                        src: nums[1],
                        dest: nums[0],
                        len: nums[2],
                    }
                })
                .collect()
        })
        .collect();

    seeds
        .iter()
        .map(|s| {
            let mut src = *s;

            for map in maps.iter() {
                for range in map {
                    if range.src <= src && src < range.src + range.len {
                        src = range.dest + src - range.src;
                        break;
                    }
                }
            }

            src
        })
        .min()
        .unwrap()
}

fn parse_seeds_part2(l: &str) -> Vec<Interval> {
    let mut res = Vec::new();

    // let t =

    for sr in l
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .chunks_exact(2)
    {
        match (sr[0].parse::<i64>(), sr[1].parse::<i64>()) {
            (Ok(s), Ok(r)) => res.push(Interval::new(s, s + r)),
            _ => unreachable!(),
        }
    }

    res
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: i64,

    /// exclusive
    end: i64,
}

impl Interval {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Copy)]
struct MapItem {
    src: Interval,
    dst: Interval,
}

fn part2(s: &str) -> i64 {
    let lines: Vec<_> = s.lines().collect();

    let mut blocks = lines.split(|l| l.is_empty());

    //
    let mut seeds = parse_seeds_part2(blocks.next().unwrap()[0]);

    println!("seed: {:?}", seeds);

    let maps: Vec<Vec<MapItem>> = blocks
        .map(|b| {
            b.iter()
                .skip(1)
                .map(|m| {
                    let nums: Vec<i64> = m
                        .split_ascii_whitespace()
                        .map(|n| n.parse().expect("Should be i64"))
                        .collect();

                    MapItem {
                        src: Interval::new(nums[1], nums[1] + nums[2]),
                        dst: Interval::new(nums[0], nums[0] + nums[2]),
                    }
                })
                .collect()
        })
        .collect();

    let mut min = i64::MAX;

    dbg!(&seeds);
    dbg!(&maps);

    for seed in seeds {
        let mut sources = VecDeque::new();
        let mut destinations = VecDeque::new();

        sources.push_back(seed);

        for map in maps.iter() {
            println!();
            println!();
            println!();
            println!();
            while !sources.is_empty() {
                println!();
                // pop from src
                // check all map item
                // if match:
                //  - push to destination
                //  - push remainder of src to src
                let to_check = sources.pop_back().unwrap();
                let mut match_found = false;

                for m_item in map {
                    println!("Checking:");
                    println!("to_check    {}..{}", to_check.start, to_check.end);
                    println!("m_item.src  {}..{}", m_item.src.start, m_item.src.end);
                    println!("m_item.dst  {}..{}", m_item.dst.start, m_item.dst.end);

                    // to_check :    *---------------*---*
                    // item     : *------------------*
                    if to_check.start > m_item.src.start
                        && to_check.start < m_item.src.end
                        && to_check.end > m_item.src.end
                    {
                        let matched_src = Interval::new(to_check.start, m_item.src.end);

                        // Remainder
                        // if to_check.end > m_item.src.end {
                        let remainder = Interval::new(m_item.src.end, to_check.end);
                        sources.push_back(remainder);

                        // }

                        let matched_dst = Interval::new(
                            m_item.dst.start + to_check.start - m_item.src.start,
                            m_item.dst.end,
                        );

                        println!("\tMatched 1 to_check: {:?} item {:?}", to_check, m_item);
                        println!("\tRemainder {:?}", remainder);
                        println!("\tMatched src {:?}", matched_src);
                        println!("\tMatched dst {:?}", matched_dst);

                        destinations.push_back(matched_dst);
                        match_found = true;
                        break;
                    }

                    // to_check :    *-----*-----------*
                    // item     :          *------------------*
                    if to_check.start < m_item.src.start
                        && to_check.end > m_item.src.start
                        && to_check.end < m_item.src.end
                    {
                        let matched_src = Interval::new(m_item.src.start, to_check.end);

                        // Remainder
                        // if to_check.end > m_item.src.end {
                        let remainder = Interval::new(to_check.start, m_item.src.start);
                        sources.push_back(remainder);

                        // }

                        let matched_dst = Interval::new(
                            m_item.dst.start,
                            m_item.dst.start + matched_src.end - matched_src.start,
                        );

                        println!("\tMatched 2 to_check: {:?} item {:?}", to_check, m_item);
                        println!("\tRemainder {:?}", remainder);
                        println!("\tMatched src {:?}", matched_src);
                        println!("\tMatched dst {:?}", matched_dst);

                        destinations.push_back(matched_dst);
                        match_found = true;
                        break;
                    }

                    // to_check :    *-----*------------------*-------*
                    // item     :          *------------------*
                    if to_check.start < m_item.src.start && to_check.end > m_item.src.end {
                        let matched_src = Interval::new(m_item.src.start, m_item.src.end);

                        // Remainder
                        // if to_check.end > m_item.src.end {
                        let remainder1 = Interval::new(to_check.start, m_item.src.start);
                        sources.push_back(remainder1);

                        let remainder2 = Interval::new(m_item.src.end, to_check.end);
                        sources.push_back(remainder2);

                        // }

                        let matched_dst = Interval::new(m_item.dst.start, m_item.dst.end);

                        println!("\tMatched 3 to_check: {:?} item {:?}", to_check, m_item);
                        println!("\tRemainder1 {:?}", remainder1);
                        println!("\tRemainder2 {:?}", remainder2);
                        println!("\tMatched src {:?}", matched_src);
                        println!("\tMatched dst {:?}", matched_dst);

                        destinations.push_back(matched_dst);
                        match_found = true;
                        break;
                    }

                    // to_check :                *------------*
                    // to_check :          *------------*
                    // to_check :             *------------*
                    // item     :          *------------------*
                    if to_check.start >= m_item.src.start && to_check.end <= m_item.src.end {
                        let matched_src = Interval::new(to_check.start, to_check.end);

                        // No remainder in this cases

                        let matched_dst = Interval::new(
                            m_item.dst.start + to_check.start - m_item.src.start,
                            m_item.dst.end - (m_item.src.end - to_check.end),
                        );

                        println!("\tMatched 4 to_check: {:?} item {:?}", to_check, m_item);
                        println!("\tMatched src {:?}", matched_src);
                        println!("\tMatched dst {:?}", matched_dst);

                        destinations.push_back(matched_dst);
                        match_found = true;
                        break;
                    }
                }

                if !match_found {
                    println!("No match found for to_check: {:?}", to_check);

                    destinations.push_back(to_check);
                }
            }

            // swap
            sources.append(&mut destinations);
            assert!(destinations.is_empty());
        }

        println!("Locations for {:?}: {:?}", seed, sources);

        min = i64::min(min, sources.iter().map(|i| i.start).min().unwrap());

        // locations
    }

    min
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works_part1() {
        let result = part1(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, 35);
    }

    #[test]
    fn it_works_part2() {
        let result = part2(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, 46);
    }
}
