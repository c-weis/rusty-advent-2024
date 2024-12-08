use regex::{Captures, Regex};
use rusty_advent_2024::utils::{self, lines_from_file};

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input03.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input03.txt"));
}

fn compute_sum(row: &str) -> i32 {
    let pattern: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Regex pattern invalid.");
    pattern
        .captures_iter(&row)
        .map(|captures| -> (i32, i32) {
            (
                captures
                    .get(1)
                    .expect("Failed to capture group 1.")
                    .as_str()
                    .parse::<i32>()
                    .expect("Failed to parse match 1."),
                captures
                    .get(2)
                    .expect("Failed to capture group 2.")
                    .as_str()
                    .parse::<i32>()
                    .expect("Failed to parse match 2."),
            )
        })
        .map(|(num1, num2)| num1 * num2)
        .sum()
}

fn part1(path: &str) -> i32 {
    lines_from_file(path)
        .map(|line| compute_sum(line.unwrap().as_str()))
        .sum()
}

fn part2(_path: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(compute_sum("mul(100,002)") == 200);
        assert!(compute_sum("mul (100,002)lkdsjflshalasjf") == 0);
        assert!(compute_sum("mul(mul(10,7)40,200)mul(10,3)") == 100);
        assert!(part1("input/input03.txt.test1") == 161);
    }

    #[test]
    fn test_part2() {
        assert!(part2("input/input03.txt.test1") == 0);
    }
}