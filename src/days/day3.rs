use regex::Regex;

use crate::util::Problem;

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};

pub fn part1(lines: &[String]) -> String {
    let l = lines.join("");
    let r = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let number_regex = Regex::new(r"[0-9]{1,3}").unwrap();
    r.find_iter(&l)
        .map(|x| {
            number_regex
                .find_iter(x.as_str())
                .map(|y| y.as_str().parse::<usize>().unwrap())
                .product::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(lines: &[String]) -> String {
    let l = lines.join("");
    let mut dont_blocks = l.split("don't()");
    //we want to keep first block as it's assumed do()
    let do_blocks = dont_blocks.next().unwrap().to_owned()
        + &dont_blocks
            .map(|x| x.split_once("do()").unwrap_or(("", "")).1)
            .collect::<Vec<&str>>()
            .join("");

    let r = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let number_regex = Regex::new(r"[0-9]{1,3}").unwrap();
    r.find_iter(&do_blocks)
        .map(|x| {
            number_regex
                .find_iter(x.as_str())
                .map(|y| y.as_str().parse::<usize>().unwrap())
                .product::<usize>()
        })
        .sum::<usize>()
        .to_string()
}
pub fn test_data() -> &'static str {
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
}
