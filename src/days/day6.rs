use crate::util::{Direction, Problem};

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};
struct Data {
    obstacles: Vec<Vec<bool>>,
    touched: Vec<Vec<bool>>,
    initial: Guard,
}
impl Data {
    fn step(&mut self) -> bool {
        let (y, x) = self.initial.yx;
        self.touched[y][x] = true;
        let (dy, dx) = self.initial.dir.to_offset();
        let (y, x) = (y as isize + dy, x as isize + dx);
        if x < 0
            || y < 0
            || x >= self.obstacles[0].len() as isize
            || y >= self.obstacles.len() as isize
        {
            return true;
        }
        if self.obstacles[y as usize][x as usize] {
            self.initial.dir = self.initial.dir.turn_right();
        } else {
            self.initial.yx = (y as usize, x as usize);
        }
        false
    }

    fn count_touched(&self) -> usize {
        self.touched.iter().flatten().filter(|&&b| b).count()
    }
}
struct Guard {
    yx: (usize, usize),
    dir: Direction,
}

pub fn part1(lines: &[String]) -> String {
    let mut d = import(lines);
    while !d.step() {}
    //print obstacles
    for l in d.obstacles.iter() {
        for c in l.iter() {
            print!("{}", if *c { '#' } else { '.' });
        }
        println!();
    }
    //print visited
    for l in d.touched.iter() {
        for c in l.iter() {
            print!("{}", if *c { 'X' } else { '.' });
        }
        println!();
    }
    d.count_touched().to_string()
}

pub fn part2(lines: &[String]) -> String {
    let _ = import(lines);
    "".to_owned()
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
    Data {
        obstacles: lines
            .iter()
            .map(|l| l.trim().chars().map(|c| c == '#').collect())
            .collect(),
        touched: lines
            .iter()
            .map(|l| l.trim().chars().map(|c| c == '^').collect())
            .collect(),
        initial: Guard {
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
        },
    }
}
