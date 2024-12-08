use crate::util::Problem;

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};
struct Data {
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

fn import(lines: &[String]) -> Data {
    Data {
        data: lines.iter().map(|x| x.trim().parse().unwrap()).collect(),
    }
}
