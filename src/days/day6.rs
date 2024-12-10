use std::collections::HashSet;

use crate::util::{Direction, Problem};

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};

#[derive(Clone)]
struct Data {
    obstacles: Vec<Vec<bool>>,
    state: Vec<Vec<Option<Direction>>>,
    current: Guard,
    initial: Guard,
}
impl Data {
    /// Takes a step and returns if we are finished, and whether or not we are
    /// in a loop (Some(true) = loop, Some(false) = out of bounds, None = neither)
    fn step(&mut self) -> Option<bool> {
        let (y, x) = self.current.yx;
        let (dy, dx) = self.current.dir.get_offset();
        let (ny, nx) = (y as isize + dy, x as isize + dx);
        if nx < 0
            || ny < 0
            || nx >= self.obstacles[0].len() as isize
            || ny >= self.obstacles.len() as isize
        {
            //OOB
            return Some(false);
        }
        let (ny, nx) = (ny as usize, nx as usize);
        if self.state[ny][nx].is_some() && self.state[ny][nx].unwrap() == self.current.dir {
            return Some(true);
        }
        if self.obstacles[ny][nx] {
            self.current.dir = self.current.dir.turn_right();
        } else {
            self.state[ny][nx] = Some(self.current.dir);
            self.current.yx = (ny, nx);
        }
        None
    }
    #[allow(dead_code)]
    fn print_state(&self) {
        println!("Current: {:?}", self.current);
        for (y, l) in self.state.iter().enumerate() {
            for (x, c) in l.iter().enumerate() {
                let is_obstacle = self.obstacles[y][x];
                print!(
                    "{}",
                    if c.is_some() {
                        if is_obstacle {
                            panic!("Obstacle and visited");
                        }
                        c.unwrap().get_icon()
                    } else if is_obstacle {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
    }
    fn step_until_done(&mut self) -> bool {
        while self.step().is_none() {}
        self.step().unwrap()
    }
    fn reset(&mut self) {
        self.state = vec![vec![None; self.obstacles[0].len()]; self.obstacles.len()];
        self.current = self.initial;
        self.state[self.initial.yx.0][self.initial.yx.1] = Some(self.initial.dir);
    }
    fn potential_loop_obstacles(&self) -> Vec<(usize, usize)> {
        let obstacles: Vec<(usize, usize)> = self
            .obstacles
            .iter()
            .enumerate()
            .filter_map(|(y, l)| {
                let got: Vec<(usize, usize)> = l
                    .iter()
                    .enumerate()
                    .filter_map(|(x, &b)| if b { Some((y, x)) } else { None })
                    .collect();
                if got.is_empty() {
                    None
                } else {
                    Some(got)
                }
            })
            .flatten()
            .collect();
        let ys: HashSet<usize> = obstacles.iter().map(|(y, _)| *y).collect();
        let yd: HashSet<usize> = ys
            .iter()
            .flat_map(|y| [*y as isize - 1, *y as isize + 1])
            .collect::<Vec<isize>>()
            .iter()
            .filter_map(|y| {
                if *y >= 0 && *y < self.obstacles.len() as isize {
                    Some(*y as usize)
                } else {
                    None
                }
            })
            .collect();
        let xs: HashSet<usize> = obstacles.iter().map(|(_, x)| *x).collect();
        let xd: HashSet<usize> = xs
            .iter()
            .flat_map(|x| [*x as isize - 1, *x as isize + 1])
            .collect::<Vec<isize>>()
            .iter()
            .filter_map(|x| {
                if *x >= 0 && *x < self.obstacles.len() as isize {
                    Some(*x as usize)
                } else {
                    None
                }
            })
            .collect();
        //ensure we will then colide after turning
        // yd.retain(|y| !xs.contains(y));
        // xd.retain(|x| !ys.contains(x));
        //join yd and xd
        let result: Vec<(usize, usize)> = yd
            .iter()
            .flat_map(|y| xd.iter().map(|x| (*y, *x)))
            //not start position
            .filter(|(y, x)| self.initial.yx != (*y, *x))
            //make sure we have an obstacle after hitting
            .filter(|(y, x)| ys.contains(y) || xs.contains(x))
            .collect();
        result
    }

    fn count_touched(&self) -> usize {
        self.state
            .iter()
            .flatten()
            .filter(|&&b| b.is_some())
            .count()
    }
}
#[derive(Clone, Copy, Debug)]
struct Guard {
    yx: (usize, usize),
    dir: Direction,
}

pub fn part1(lines: &[String]) -> String {
    let mut d = import(lines);
    d.step_until_done();
    // d.print_state();
    d.count_touched().to_string()
}

pub fn part2(lines: &[String]) -> String {
    let mut d = import(lines);
    d.step_until_done();
    let mut potential = d.potential_loop_obstacles();
    potential.retain(|(y, x)| d.state[*y][*x].is_some());
    potential.retain(|(y, x)| {
        let mut dp = d.clone();
        dp.obstacles[*y][*x] = true;
        dp.reset();
        dp.step_until_done()
    });
    potential.len().to_string()
}
pub fn test_data() -> &'static str {
    "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
}

fn import(lines: &[String]) -> Data {
    let lines: Vec<&String> = lines.iter().filter(|l| !l.is_empty()).collect();
    let initial = Guard {
        yx: lines
            .iter()
            .enumerate()
            .find_map(|(y, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .find(|&(_, c)| c == '^')
                    .map(|(x, _)| (y, x))
            })
            .unwrap(),
        dir: Direction::Up,
    };
    let mut state = vec![vec![None; lines[0].len()]; lines.len()];
    state[initial.yx.0][initial.yx.1] = Some(initial.dir);
    Data {
        obstacles: lines
            .iter()
            .map(|l| l.trim().chars().map(|c| c == '#').collect())
            .collect(),
        state,
        initial,
        current: initial,
    }
}
