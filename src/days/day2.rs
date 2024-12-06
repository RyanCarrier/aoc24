use crate::util::Problem;

pub const DAY2: Problem = Problem {
    day: 2,
    part1,
    part2,
    test_data: Some(test_data),
};
struct Day2Data {
    data: Vec<i32>,
}

pub fn part1(lines: &[String]) -> String {
    let _ = import(lines);
    "".to_owned()
}

pub fn part2(lines: &[String]) -> String {
    let _ = import(lines);
    "".to_owned()
}
pub fn test_data() -> &'static str {
    ""
}

fn import(lines: &[String]) -> Day2Data {
    Day2Data {
        data: lines.iter().map(|x| x.trim().parse().unwrap()).collect(),
    }
}
