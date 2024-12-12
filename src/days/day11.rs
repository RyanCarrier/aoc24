use cached::proc_macro::cached;
use num::Integer;

use crate::util::Problem;

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};

fn step_all(stones: Vec<usize>, steps: usize) -> usize {
    stones.iter().fold(0, |acc, &x| acc + step(x, steps))
}
#[cached]
fn step(x: usize, steps: usize) -> usize {
    if steps == 0 {
        return 1;
    }
    if x == 0 {
        step(1, steps - 1)
    } else {
        let l = x.checked_ilog10().unwrap_or(0) + 1;
        if l.is_even() {
            let m = 10_usize.pow(l / 2);
            let lhs = x / m;
            let rhs = x % m;
            step(lhs, steps - 1) + step(rhs, steps - 1)
        } else {
            step(x * 2024, steps - 1)
        }
    }
}

pub fn part1(lines: &[String]) -> String {
    let d = import(lines);
    step_all(d, 25).to_string()
}

pub fn part2(lines: &[String]) -> String {
    let d = import(lines);
    step_all(d, 75).to_string()
}
pub fn test_data() -> &'static str {
    // "0"
    "125 17"
}

fn import(lines: &[String]) -> Vec<usize> {
    lines
        .iter()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}
