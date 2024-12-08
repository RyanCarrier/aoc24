use std::collections::HashSet;

use crate::util::Problem;

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};
struct Data {
    rules: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

impl Data {
    fn is_update_correct(&self, update: &[usize]) -> bool {
        self.first_failed_rule(update).is_none()
    }
    fn first_failed_rule(&self, update: &[usize]) -> Option<(usize, usize)> {
        for rule in self.rules.iter() {
            //find index of rule.0 in update
            let a = update.iter().position(|x| x == &rule.0).unwrap_or(0);
            let b = update
                .iter()
                .position(|x| x == &rule.1)
                .unwrap_or(update.len());
            if a > b {
                return Some(*rule);
            }
        }
        None
    }
    fn get_correct_updates(&self) -> Vec<&Vec<usize>> {
        self.updates
            .iter()
            .filter(|x| self.is_update_correct(x))
            .collect()
    }
    fn get_incorrect_updates(&self) -> Vec<&Vec<usize>> {
        self.updates
            .iter()
            .filter(|x| !self.is_update_correct(x))
            .collect()
    }
}

pub fn part1(lines: &[String]) -> String {
    let d = import(lines);
    d.get_correct_updates()
        .into_iter()
        .map(|x| x[x.len() / 2])
        .sum::<usize>()
        .to_string()
}

pub fn part2(lines: &[String]) -> String {
    let d = import(lines);
    d.get_incorrect_updates()
        .iter()
        .map(|u| {
            let mut u = (*u).clone();
            while let Some(r) = d.first_failed_rule(&u) {
                let a = u.iter().position(|x| *x == r.0).unwrap();
                let b = u.iter().position(|x| *x == r.1).unwrap();
                if a > b {
                    u.swap(a, b);
                }
                //hopefully 2 rules don't colide and we just loop forever
            }
            u
        })
        .map(|x| x[x.len() / 2])
        .sum::<usize>()
        .to_string()
}
pub fn test_data() -> &'static str {
    "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
}

fn import(lines: &[String]) -> Data {
    let mut split = lines.split(|l| l.is_empty());
    let mut rules: Vec<(usize, usize)> = split
        .next()
        .unwrap()
        .iter()
        .map(|l| {
            let raw = l.split_once('|').unwrap();
            (
                raw.0.parse::<usize>().unwrap(),
                raw.1.parse::<usize>().unwrap(),
            )
        })
        .collect();
    let updates: Vec<Vec<usize>> = split
        .next()
        .unwrap()
        .iter()
        .map(|l| l.split(',').map(|x| x.parse::<usize>().unwrap()).collect())
        .collect();
    let all_docs = updates
        .clone()
        .into_iter()
        .flatten()
        .collect::<HashSet<usize>>();
    rules.retain(|(a, b)| all_docs.contains(a) && all_docs.contains(b));
    Data { rules, updates }
}
