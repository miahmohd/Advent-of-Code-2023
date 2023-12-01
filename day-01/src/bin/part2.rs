static MATCHES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let s = include_str!("./input2.txt");

    let res = run(s);

    println!("{res}");
}

fn run(s: &str) -> u32 {
    let mut result = 0;

    for line in s.lines() {
        let digits = find_digit(&line, Vec::new());

        if let (Some(d), Some(u)) = (digits.first(), digits.last()) {
            result += d * 10 + u;
        }
    }

    return result;
}

fn find_digit(s: &str, mut digits: Vec<u32>) -> Vec<u32> {
    if s.is_empty() {
        return digits;
    };

    if let Some(c) = s.chars().next() {
        if c.is_ascii_digit() {
            digits.push(c.to_digit(10).unwrap());
        }
    }

    let mut i = 0;

    for m in MATCHES.iter() {
        if s.starts_with(m) {
            digits.push(i);
        }
        i += 1;
    }

    return find_digit(&s[1..], digits);
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn it_works() {
        let result = run("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, 281);
    }
}
