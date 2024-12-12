use crate::util::{Direction, Problem};
use rayon::prelude::*;

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};
struct Data {
    plots: Vec<Vec<u8>>,
}
impl Data {
    fn get_safe(&self, pos: (isize, isize)) -> u8 {
        if pos.0 < 0
            || pos.1 < 0
            || pos.0 >= self.plots.len() as isize
            || pos.1 >= self.plots[0].len() as isize
        {
            return u8::MAX;
        }
        self.plots[pos.0 as usize][pos.1 as usize]
    }
    fn get_perimeters(&self) -> Vec<Vec<u8>> {
        self.plots
            .par_iter()
            .enumerate()
            .map(|(y, row)| {
                let y = y as isize;
                row.iter()
                    .enumerate()
                    .map(|(x, c)| {
                        let x = x as isize;
                        Direction::iter()
                            .map(|dir| self.get_safe(dir.map_offset((y, x))))
                            .filter(|neighbour_c| neighbour_c != c)
                            .count() as u8
                    })
                    .collect()
            })
            .collect()
    }
    /// returns (char, area, perimeter)
    fn get_plot(
        &self,
        touched: &mut [Vec<bool>],
        perimeters: &[Vec<u8>],
        pos: (isize, isize),
    ) -> (u8, usize, usize) {
        let mut queue = vec![pos];
        let mut area = 0;
        let mut perimeter = 0_usize;
        let c = self.plots[pos.0 as usize][pos.1 as usize];
        touched[pos.0 as usize][pos.1 as usize] = true;
        while let Some(current) = queue.pop() {
            area += 1;
            perimeter += perimeters[current.0 as usize][current.1 as usize] as usize;
            for dir in Direction::iter() {
                let next = dir.map_offset(current);
                if self.get_safe(next) != c || touched[next.0 as usize][next.1 as usize] {
                    continue;
                }
                queue.push(next);
                touched[next.0 as usize][next.1 as usize] = true;
            }
        }
        (c, area, perimeter)
    }
    fn get_fence_prices(&self) -> Vec<(u8, usize)> {
        let perimeters = self.get_perimeters();
        let mut touched = vec![vec![false; self.plots[0].len()]; self.plots.len()];
        let mut y = 0;
        let mut x = 0;
        let mut prices = vec![];
        while y < self.plots.len() {
            while x < self.plots[0].len() {
                if !touched[y][x] {
                    prices.push(self.get_plot(&mut touched, &perimeters, (y as isize, x as isize)));
                }
                x += 1;
            }
            y += 1;
            x = 0;
        }
        prices.into_iter().map(|(c, a, p)| (c, a * p)).collect()
    }
}

pub fn part1(lines: &[String]) -> String {
    let d = import(lines);
    d.get_fence_prices()
        .into_iter()
        .map(|(_, price)| price)
        .sum::<usize>()
        .to_string()
}

pub fn part2(lines: &[String]) -> String {
    let _ = import(lines);
    "".to_owned()
}
pub fn test_data() -> &'static str {
    "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
}

fn import(lines: &[String]) -> Data {
    Data {
        plots: lines
            .iter()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().map(|c| c as u8 - b'A').collect())
            .collect(),
    }
}
