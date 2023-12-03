use std::collections::{BTreeMap, HashSet};

fn main() {
    let res = part1(include_str!("./input1"));
    println!("Part 1: {res}");

    let res = part2(include_str!("./input2"));
    println!("Part 2: {res}");
}

trait IsSymb {
    fn is_symbol(&self) -> bool;
    fn is_gear(&self) -> bool;
}

impl IsSymb for char {
    fn is_symbol(&self) -> bool {
        return *self != '.' && !self.is_ascii_digit();
    }

    fn is_gear(&self) -> bool {
        return *self == '*';
    }
}

fn part1(s: &str) -> i32 {
    let lines: Vec<&str> = s.lines().collect();

    let mut parts: Vec<i32> = Vec::new();

    for i in 0..lines.len() {
        //
        let middle = lines.get(i).unwrap();
        let top = if i == 0 { None } else { lines.get(i - 1) };
        let bottom = if i == lines.len() - 1 {
            None
        } else {
            lines.get(i + 1)
        };

        let mut temp_part = String::new();
        let mut have_found_ad_sym = false;

        for (m_idx, c) in middle.chars().enumerate() {
            match c {
                c if c.is_ascii_digit() => {
                    temp_part.push(c);

                    // Check for top symbols
                    if let Some(top) = top {
                        let top: Vec<char> = top.chars().collect();
                        let t_left = if m_idx == 0 { top[0] } else { top[m_idx - 1] };
                        let t_right = if m_idx == top.len() - 1 {
                            top[top.len() - 1]
                        } else {
                            top[m_idx + 1]
                        };
                        have_found_ad_sym = have_found_ad_sym
                            || t_left.is_symbol()
                            || top[m_idx].is_symbol()
                            || t_right.is_symbol();
                    }

                    // Check for bottom symbols
                    if let Some(bottom) = bottom {
                        let bottom: Vec<char> = bottom.chars().collect();
                        let b_left = if m_idx == 0 {
                            bottom[0]
                        } else {
                            bottom[m_idx - 1]
                        };
                        let b_right = if m_idx == bottom.len() - 1 {
                            bottom[bottom.len() - 1]
                        } else {
                            bottom[m_idx + 1]
                        };
                        have_found_ad_sym = have_found_ad_sym
                            || b_left.is_symbol()
                            || bottom[m_idx].is_symbol()
                            || b_right.is_symbol();
                    }

                    if have_found_ad_sym && m_idx == middle.len() - 1 {
                        parts.push(temp_part.parse().expect("Should be valid number"));
                        temp_part.clear();
                        have_found_ad_sym = false;
                    }
                }
                '.' => {
                    if have_found_ad_sym && !temp_part.is_empty() {
                        parts.push(temp_part.parse().expect("Should be valid number"));
                    } else if !temp_part.is_empty() {
                    }
                    temp_part.clear();
                    have_found_ad_sym = false;
                }
                _ if c.is_symbol() => {
                    if !temp_part.is_empty() {
                        parts.push(temp_part.parse().expect("Should be valid number"));
                    }
                    temp_part.clear();
                    have_found_ad_sym = true
                }
                _ => unreachable!(),
            }
        }
    }

    parts.iter().sum()
}

#[derive(Debug, Default)]
struct GearPart {
    part: String,
    sym_pos: (usize, usize),
}

fn part2(s: &str) -> i32 {
    let lines: Vec<&str> = s.lines().collect();

    let mut gear_parts: BTreeMap<(usize, usize), Vec<i32>> = BTreeMap::new();

    for i in 0..lines.len() {
        //
        let middle = lines.get(i).unwrap();
        let top = if i == 0 { None } else { lines.get(i - 1) };
        let bottom = if i == lines.len() - 1 {
            None
        } else {
            lines.get(i + 1)
        };

        let mut tmp_part = String::new();
        let mut tmp_part_gear_pos: HashSet<(usize, usize)> = HashSet::new();
        let mut have_found_adj_gear = false;

        for (m_idx, c) in middle.chars().enumerate() {
            match c {
                c if c.is_ascii_digit() => {
                    tmp_part.push(c);

                    // Check for top symbols
                    if let Some(top) = top {
                        let top: Vec<char> = top.chars().collect();
                        // let top_line_idx = if i == 0 { -1 } else { i };
                        let t_left = if m_idx == 0 { top[0] } else { top[m_idx - 1] };
                        let t_right = if m_idx == top.len() - 1 {
                            top[top.len() - 1]
                        } else {
                            top[m_idx + 1]
                        };
                        have_found_adj_gear = have_found_adj_gear
                            || t_left.is_gear()
                            || top[m_idx].is_gear()
                            || t_right.is_gear();

                        if t_left.is_gear() {
                            tmp_part_gear_pos
                                .insert((i - 1, if m_idx == 0 { 0 } else { m_idx - 1 }));
                        }

                        if top[m_idx].is_gear() {
                            tmp_part_gear_pos.insert((i - 1, m_idx));
                        }

                        if t_right.is_gear() {
                            tmp_part_gear_pos.insert((
                                i - 1,
                                if m_idx == top.len() - 1 {
                                    top.len() - 1
                                } else {
                                    m_idx + 1
                                },
                            ));
                        }
                    }

                    // Check for bottom symbols
                    if let Some(bottom) = bottom {
                        let bottom: Vec<char> = bottom.chars().collect();
                        let b_left = if m_idx == 0 {
                            bottom[0]
                        } else {
                            bottom[m_idx - 1]
                        };
                        let b_right = if m_idx == bottom.len() - 1 {
                            bottom[bottom.len() - 1]
                        } else {
                            bottom[m_idx + 1]
                        };
                        have_found_adj_gear = have_found_adj_gear
                            || b_left.is_gear()
                            || bottom[m_idx].is_gear()
                            || b_right.is_gear();

                        if b_left.is_gear() {
                            tmp_part_gear_pos
                                .insert((i + 1, if m_idx == 0 { 0 } else { m_idx - 1 }));
                        }

                        if bottom[m_idx].is_gear() {
                            tmp_part_gear_pos.insert((i + 1, m_idx));
                        }

                        if b_right.is_gear() {
                            tmp_part_gear_pos.insert((
                                i + 1,
                                if m_idx == bottom.len() - 1 {
                                    bottom.len() - 1
                                } else {
                                    m_idx + 1
                                },
                            ));
                        }
                    }

                    if have_found_adj_gear && m_idx == middle.len() - 1 {
                        let gear_part: i32 = tmp_part.parse().expect("Should be valid number");
                        println!("gerar pos: {:?}", tmp_part_gear_pos);
                        for pos in tmp_part_gear_pos {
                            if gear_parts.contains_key(&pos) {
                                gear_parts.get_mut(&pos).unwrap().push(gear_part);
                            } else {
                                gear_parts.insert(pos, vec![gear_part]);
                            }
                        }
                        tmp_part.clear();
                        tmp_part_gear_pos = HashSet::new();
                        have_found_adj_gear = false;
                    }
                }

                '*' => {
                    tmp_part_gear_pos.insert((i, m_idx));
                    if !tmp_part.is_empty() {
                        let gear_part: i32 = tmp_part.parse().expect("Should be valid number");
                        println!("gerar pos: {:?}", tmp_part_gear_pos);

                        for pos in tmp_part_gear_pos.iter() {
                            if gear_parts.contains_key(&pos) {
                                gear_parts.get_mut(&pos).unwrap().push(gear_part);
                            } else {
                                gear_parts.insert(*pos, vec![gear_part]);
                            }
                        }
                    }
                    tmp_part.clear();
                    // tmp_part_gear_pos = HashSet::new();
                    have_found_adj_gear = true;
                }
                _ => {
                    if have_found_adj_gear && !tmp_part.is_empty() {
                        let gear_part: i32 = tmp_part.parse().expect("Should be valid number");
                        println!("gerar pos: {:?}", tmp_part_gear_pos);

                        for pos in tmp_part_gear_pos {
                            if gear_parts.contains_key(&pos) {
                                gear_parts.get_mut(&pos).unwrap().push(gear_part);
                            } else {
                                gear_parts.insert(pos, vec![gear_part]);
                            }
                        }
                    }
                    tmp_part.clear();
                    tmp_part_gear_pos = HashSet::new();
                    have_found_adj_gear = false;
                }
            }
        }
    }

    println!(
        "Parts key ({}) {:?}",
        gear_parts.keys().len(),
        gear_parts.keys().collect::<Vec<_>>()
    );
    println!();
    println!();
    println!();
    println!("Parts val {:?}", gear_parts.values().collect::<Vec<_>>());

    gear_parts
        .values()
        .filter(|p| p.len() == 2)
        .map(|p| p[0] * p[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works_part1() {
        let result = part1(
            "467..114..
...*......
..35..633.
...*....#.
.617......
......58..
.+592.....
.....*755.
...$......
.664+.598$",
        );
        assert_eq!(result, 4361);
    }

    #[test]
    fn test() {
        let result = part1(
            ".........
......+11
.........",
        );
        assert_eq!(result, 11);
    }

    #[test]
    fn it_works_part2() {
        let result = part2(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 467835);
    }

    #[test]
    fn it_debug_part2() {
        let result = part2(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
..........
..........
.755*598..",
        );
        assert_eq!(result, 467835);
    }
}
