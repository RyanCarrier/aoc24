use crate::util::{Direction, Problem, TRANSFORMS_DIAG};
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
    fn get_plot2(&self, touched: &mut [Vec<bool>], pos: (isize, isize)) -> (u8, usize, usize) {
        let mut queue = vec![pos];
        let mut area = 0;
        let mut corners = 0_usize;
        let c = self.plots[pos.0 as usize][pos.1 as usize];
        touched[pos.0 as usize][pos.1 as usize] = true;
        while let Some(current) = queue.pop() {
            area += 1;
            let friends: Vec<Option<(isize, isize)>> = Direction::iter()
                .map(|d| {
                    let next = d.map_offset(current);
                    if self.get_safe(next) == c {
                        Some(next)
                    } else {
                        None
                    }
                })
                .collect();
            let friend_count = friends.iter().filter(|f| f.is_some()).count();
            corners += match friend_count {
                0 => 4,
                1 => 2,
                //if we have 4 friends, check if we are a internal corner
                // i think this might fail if we have 3 friends and a internal corner
                4 => TRANSFORMS_DIAG
                    .iter()
                    .filter(|&&t| self.get_safe((current.0 + t.0, current.1 + t.1)) != c)
                    .count(),
                2 => {
                    let mut local_friends = friends.iter().filter(|x| x.is_some());
                    //god i hate this
                    let (a, b) = (
                        local_friends.next().unwrap().unwrap(),
                        local_friends.next().unwrap().unwrap(),
                    );
                    //check for internal corner
                    if a.0 == b.0 || a.1 == b.1 {
                        //we are on a straight line
                        0
                    } else if current.0 == a.0 {
                        // use x of a and y of b
                        if self.get_safe((b.0, a.1)) == c {
                            1
                        } else {
                            2
                        }
                        // use opposite
                    } else if self.get_safe((a.0, b.1)) == c {
                        1
                    } else {
                        2
                    }
                }
                3 => {
                    let i = (friends.iter().position(|x| x.is_none()).unwrap() + 2) % 4;
                    let diag_offsets = Direction::get_directions()[i].get_diagonal_offsets();
                    diag_offsets
                        .iter()
                        .filter(|&&t| self.get_safe((current.0 + t.0, current.1 + t.1)) != c)
                        .count()
                }
                _ => panic!("unexpected friend count {}", friend_count),
            };
            friends.iter().filter_map(|&x| x).for_each(|next| {
                if !touched[next.0 as usize][next.1 as usize] {
                    queue.push(next);
                    touched[next.0 as usize][next.1 as usize] = true;
                }
            });
        }
        (c, area, corners)
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
    fn get_fence_prices2(&self) -> Vec<(u8, usize)> {
        let mut touched = vec![vec![false; self.plots[0].len()]; self.plots.len()];
        let mut y = 0;
        let mut x = 0;
        let mut prices = vec![];
        while y < self.plots.len() {
            while x < self.plots[0].len() {
                if !touched[y][x] {
                    prices.push(self.get_plot2(&mut touched, (y as isize, x as isize)));
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
    let d = import(lines);
    d.get_fence_prices2()
        .into_iter()
        .map(|(_, price)| price)
        .sum::<usize>()
        .to_string()
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
