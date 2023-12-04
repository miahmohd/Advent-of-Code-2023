fn main() {
    let res = part1(include_str!("./input1"));
    println!("Part 1: {res}");

    // let res = part2(include_str!("./input2"));
    // println!("Part 2: {res}");
}

fn part1(s: &str) -> i32 {
    0
}

fn part2(s: &str) -> i32 {
    // Parse cards in BTreeMap

    0
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works_part1() {
        let result = part1("");
        assert_eq!(result, 13);
    }

    #[test]
    fn it_works_part2() {
        let result = part2("");
        assert_eq!(result, 30);
    }
}
