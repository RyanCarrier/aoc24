use std::collections::HashSet;

use crate::util::{Direction, Problem};

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};
struct Data {
    map: Vec<Vec<u8>>,
    max: (isize, isize),
}
impl Data {
    fn walk(&self, p: (isize, isize)) -> Vec<(isize, isize)> {
        let c = self.map[p.0 as usize][p.1 as usize];
        if c == 9 {
            return vec![p];
        }
        Direction::iter()
            .map(|d| d.get_offset())
            .filter_map(|(dy, dx)| {
                let pn = (p.0 + dy, p.1 + dx);
                if pn.0 < 0 || pn.1 < 0 || pn.0 > self.max.0 || pn.1 > self.max.1 {
                    return None;
                }
                let cn = self.map[pn.0 as usize][pn.1 as usize];
                if cn == c + 1 {
                    let got = self.walk(pn);
                    if !got.is_empty() {
                        return Some(got);
                    }
                }
                None
            })
            .flatten()
            .collect()
    }
    fn zeros(&self) -> Vec<(usize, usize)> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .filter(|(_, &c)| c == 0)
                    .map(|(x, _)| (y, x))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect()
    }
    fn walk_zeros(&self) -> usize {
        self.zeros()
            .iter()
            .map(|(y, x)| {
                self.walk((*y as isize, *x as isize))
                    .iter()
                    .collect::<HashSet<&(isize, isize)>>()
                    .len()
            })
            .sum()
    }
    fn walk_zeros2(&self) -> usize {
        self.zeros()
            .iter()
            .map(|(y, x)| self.walk((*y as isize, *x as isize)).len())
            .sum()
    }
}

pub fn part1(lines: &[String]) -> String {
    let data = import(lines);
    data.walk_zeros().to_string()
}

pub fn part2(lines: &[String]) -> String {
    let data = import(lines);
    data.walk_zeros2().to_string()
}
pub fn test_data() -> &'static str {
    "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
}

fn import(lines: &[String]) -> Data {
    let lines: Vec<&String> = lines.iter().filter(|l| !l.is_empty()).collect();
    Data {
        max: ((lines.len() - 1) as isize, (lines[0].len() - 1) as isize),
        map: lines
            .iter()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
            .collect(),
    }
}
