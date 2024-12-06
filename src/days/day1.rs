use crate::util::Problem;

pub const DAY1: Problem = Problem {
    day: 1,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Day1Data {
    data: Vec<i32>,
}

pub fn part1(lines: &Vec<String>) -> String {
    let _ = import(lines);
    "".to_owned()
}

pub fn part2(lines: &Vec<String>) -> String {
    let _ = import(lines);
    "".to_owned()
}
pub fn test_data() -> &'static str {
    ""
}

fn import(lines: &Vec<String>) -> Day0Data {
    Day1Data {
        data: lines.iter().map(|x| x.trim().parse().unwrap()).collect(),
    }
}
