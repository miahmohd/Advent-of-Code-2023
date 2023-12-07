use core::time;

fn main() {
    let res = part1(include_str!("./input"));
    println!("Part 1: {res}");

    let res = part2(include_str!("./input"));
    println!("Part 2: {res}");
}

fn part1(s: &str) -> f32 {
    let r = 1.0;

    let mut lines = s.lines();

    let times: Vec<f32> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse().expect("Should be number"))
        .collect();

    let distances: Vec<f32> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse().expect("Should be number"))
        .collect();

    dbg!(&times);
    dbg!(&distances);

    let mut res = 1.0;

    for (t, d) in times.iter().zip(distances.iter()) {
        let x1 =
            f32::floor(((r * t - f32::sqrt((r * r * t * t - 4.0 * r * d) as f32)) / 2.0 * r) + 1.0);
        let x2 =
            f32::ceil(((r * t + f32::sqrt((r * r * t * t - 4.0 * r * d) as f32)) / 2.0 * r) - 1.0);

        println!(" {} {} -> {}", x1, x2, x2 - x1 + 1.0);
        res = res * (x2 - x1 + 1.0);
    }

    res
}

fn part2(s: &str) -> f64 {
    let r = 1.0;

    let mut lines = s.lines();

    let t: f64 = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    let d: f64 = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    dbg!(&t);
    dbg!(&d);

    let mut res = 1.0;

    let x1 =
        f64::floor(((r * t - f64::sqrt((r * r * t * t - 4.0 * r * d) as f64)) / 2.0 * r) + 1.0);
    let x2 = f64::ceil(((r * t + f64::sqrt((r * r * t * t - 4.0 * r * d) as f64)) / 2.0 * r) - 1.0);

    println!(" {} {} -> {}", x1, x2, x2 - x1 + 1.0);
    res = res * (x2 - x1 + 1.0);

    res
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works_part1() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 288.0);
    }

    #[test]
    fn it_works_part2() {
        let result = part2(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 71503.0);
    }

    #[test]
    fn it_works_part2_input() {
        let result = part2(
            "Time:        63     78     94     68
Distance:   411   1274   2047   1035",
        );
        assert_eq!(result, 49240091.0);
    }
}
