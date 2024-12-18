use std::collections::{BinaryHeap, HashSet};

use crate::util::{Direction, Problem, Vertex};

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};
struct Data {
    walls: Vec<Vec<bool>>,
    start: (isize, isize),
    end: (isize, isize),
}
impl Data {
    fn yx_to_index(&self, (y, x): (isize, isize)) -> usize {
        (y * self.walls[0].len() as isize + x) as usize
    }
    fn index_to_yx(&self, index: usize) -> (isize, isize) {
        (
            (index / self.walls[0].len()) as isize,
            (index % self.walls[0].len()) as isize,
        )
    }
    fn get(&self, i: usize) -> bool {
        self.getyx(self.index_to_yx(i))
    }
    fn getyx(&self, (y, x): (isize, isize)) -> bool {
        if y < 0 || x < 0 || y >= self.walls.len() as isize || x >= self.walls[0].len() as isize {
            return true;
        }
        self.walls[y as usize][x as usize]
    }
    fn get_path_cells(&self, dist: &[usize], dirs: &[Direction]) -> Vec<(isize, isize)> {
        let turn_cost = 1000;
        let step_cost = 1;
        let fancy_step_cost = step_cost + turn_cost;
        let mut path = vec![];
        let mut need_to_check = vec![self.end];
        while let Some(current) = need_to_check.pop() {
            if current == self.start || path.contains(&current) {
                continue;
            }
            let current_index = self.yx_to_index(current);
            path.push(current);
            let current_dist = dist[current_index];
            for dir in Direction::get_directions().iter() {
                let next = dir.map_offset(current);
                if self.getyx(next) || path.contains(&next) {
                    continue;
                }
                let next_index = self.yx_to_index(next);
                let dist_diff = current_dist as isize - dist[next_index] as isize;
                if (dist_diff == fancy_step_cost && dir.opposite() == dirs[current_index])
                    || (dist_diff == step_cost && dirs[current_index] == dirs[next_index])
                {
                    //regular path
                    need_to_check.push(next);
                }
                let double_next = dir.map_offset(next);
                if *dir == dirs[current_index].opposite()
                    && dirs[current_index] != dirs[next_index]
                    && dist[self.yx_to_index(double_next)] == dist[current_index] - 2
                {
                    //there was a turn on next, check if we could have walked over
                    // it from a different place
                    need_to_check.push(double_next);
                }
            }
        }
        path.push(self.start);
        path.into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    fn run(&self) -> (Vec<usize>, Vec<Direction>) {
        let turn_cost = 1000;
        let step_cost = 1;
        let max = self.walls.len() * self.walls[0].len();
        let end_index = self.yx_to_index(self.end);
        let mut pq: BinaryHeap<Vertex> = BinaryHeap::new();
        let mut dist = vec![usize::MAX; max];
        let mut dir = vec![Direction::Right; max];
        dist[self.yx_to_index(self.start)] = 0;
        pq.push(Vertex {
            index: self.yx_to_index(self.start),
            dist: 0,
        });
        while let Some(next) = pq.pop() {
            if next.index == end_index {
                return (dist, dir);
            }
            let next_dir = dir[next.index];
            let next_dirs = [
                (dist[next.index] + step_cost, next_dir),
                (
                    dist[next.index] + turn_cost + step_cost,
                    next_dir.turn_left(),
                ),
                (
                    dist[next.index] + turn_cost + step_cost,
                    next_dir.turn_right(),
                ),
            ];
            let next_yx = self.index_to_yx(next.index);

            for (next_dist, direction) in next_dirs.iter() {
                let to = self.yx_to_index(direction.map_offset(next_yx));
                if self.get(to) {
                    //wall
                    continue;
                }
                if *next_dist < dist[to] {
                    dist[to] = *next_dist;
                    dir[to] = *direction;
                    pq.push(Vertex {
                        index: to,
                        dist: *next_dist,
                    });
                }
            }
        }
        panic!("no route found");
    }
}

pub fn part1(lines: &[String]) -> String {
    // ok this current method doesn't care which direction we are facing for distance
    // so we might overwrite better solutions for 'short term' gains
    let d = import(lines);
    let (dist, _) = d.run();
    dist[d.yx_to_index(d.end)].to_string()
}

pub fn part2(lines: &[String]) -> String {
    let d = import(lines);
    let (dist, dirs) = d.run();
    d.get_path_cells(&dist, &dirs).len().to_string()
}
pub fn test_data() -> &'static str {
    [
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
    ][1]
}

fn import(lines: &[String]) -> Data {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let walls = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (y as isize, x as isize);
                    } else if c == 'E' {
                        end = (y as isize, x as isize);
                    }
                    c == '#'
                })
                .collect()
        })
        .collect();
    Data { walls, start, end }
}
