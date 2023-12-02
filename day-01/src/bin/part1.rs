fn main() {
    let s = include_str!("./input1.txt");

    let res = run(s);

    println!("{res}");
}

fn run(s: &str) -> u32 {
    let result = s
        .lines()
        .map(|l| {
            let digits: Vec<u32> = l.chars().filter_map(|c| c.to_digit(10)).collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum();

    return result;
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn it_works() {
        let result = run("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, 142);
    }
}
