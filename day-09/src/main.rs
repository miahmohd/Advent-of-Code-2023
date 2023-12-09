use core::time;

fn main() {
    let res = part1(include_str!("./input"));
    println!("Part 1: {res}");

    let res = part2(include_str!("./input"));
    println!("Part 2: {res}");
}

fn next_step(step: &Vec<i32>) -> Vec<i32> {
    let mut next = Vec::new();

    for i in 0..step.len() - 1 {
        match (step.get(i), step.get(i + 1)) {
            (Some(a), Some(b)) => next.push(b - a),
            _ => unreachable!(),
        }
    }
    next
}

fn next_number(step: &Vec<i32>, sub_res: i32) -> i32 {
    let last = step.last().unwrap();

    sub_res + last
}

fn part1(s: &str) -> i32 {
    let lines: Vec<Vec<i32>> = s
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse().expect("should be i32"))
                .collect()
        })
        .collect();

    let mut completed_seqs = Vec::new();

    for history in lines {
        let mut steps = vec![history];

        loop {
            let last_step = steps.last().unwrap();
            let next_step = next_step(&last_step);

            if next_step.iter().all(|n| *n == 0) {
                steps.push(next_step);
                break;
            }
            steps.push(next_step);
        }

        completed_seqs.push(steps);
    }

    let mut res = 0;

    for seq in completed_seqs.iter_mut() {
        let mut last_n = 0;
        seq.last_mut().unwrap().push(0);

        for step in seq.iter_mut().rev().skip(1) {
            let next_number = next_number(&step, last_n);

            step.push(next_number);

            last_n = next_number;
        }

        // println!("last: {}", last_n);
        res += last_n;
    }

    // dbg!(completed_seqs);

    res
}

fn prev_number(step: &Vec<i32>, sub_res: i32) -> i32 {
    let first = step.first().unwrap();

    first - sub_res
}

fn part2(s: &str) -> i32 {
    let lines: Vec<Vec<i32>> = s
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse().expect("should be i32"))
                .collect()
        })
        .collect();

    let mut completed_seqs = Vec::new();

    for history in lines {
        let mut steps = vec![history];

        loop {
            let last_step = steps.last().unwrap();
            let next_step = next_step(&last_step);

            if next_step.iter().all(|n| *n == 0) {
                steps.push(next_step);
                break;
            }
            steps.push(next_step);
        }

        completed_seqs.push(steps);
    }

    let mut res = 0;

    for seq in completed_seqs.iter_mut() {
        let mut prev_n = 0;
        seq.last_mut().unwrap().insert(0, 0);

        for step in seq.iter_mut().rev().skip(1) {
            let next_number = prev_number(&step, prev_n);

            step.insert(0, next_number);

            prev_n = next_number;
        }

        // println!("last: {}", last_n);
        res += prev_n;
    }

    dbg!(completed_seqs);

    res
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn it_works_part1() {
        let result = part1(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, 114);
    }

    #[test]
    fn it_works_part2() {
        let result = part2(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, 2);
    }
}
