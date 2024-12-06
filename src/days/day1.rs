use std::collections::{HashMap, HashSet};

use crate::util::Problem;

pub const DAY1: Problem = Problem {
    day: 1,
    part1,
    part2,
    test_data: Some(test_data),
};

pub fn part1(lines: &[String]) -> String {
    //create vec of size lines.len()
    let mut left: Vec<i32> = vec![0; lines.len()];
    let mut right: Vec<i32> = vec![0; lines.len()];
    lines
        .iter()
        .filter(|l| !l.is_empty())
        .enumerate()
        .for_each(|(i, l)| {
            let mut numbers = l.split_whitespace();
            left[i] = numbers.next().unwrap().parse::<i32>().unwrap();
            right[i] = numbers.next().unwrap().parse().unwrap();
        });
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| acc + (l.abs_diff(*r) as i32))
        .to_string()
}

pub fn part2(lines: &[String]) -> String {
    let mut left: Vec<i32> = vec![0; lines.len()];
    let mut right: Vec<i32> = vec![0; lines.len()];
    lines
        .iter()
        .filter(|l| !l.is_empty())
        .enumerate()
        .for_each(|(i, l)| {
            let mut numbers = l.split_whitespace();
            left[i] = numbers.next().unwrap().parse::<i32>().unwrap();
            right[i] = numbers.next().unwrap().parse().unwrap();
        });
    left.sort();
    let left_unique: HashSet<i32> = left.into_iter().collect();
    let right_map = right
        .into_iter()
        .fold(HashMap::new(), |mut acc: HashMap<i32, usize>, n| {
            if acc.contains_key(&n) {
                acc.insert(n, acc.get(&n).unwrap() + 1);
            } else {
                acc.insert(n, 1);
            }
            acc
        });
    left_unique
        .iter()
        .fold(0, |acc, l| {
            if let Some(r) = right_map.get(l) {
                acc + (*l as usize) * r
            } else {
                acc
            }
        })
        .to_string()
}
pub fn test_data() -> &'static str {
    ""
}
