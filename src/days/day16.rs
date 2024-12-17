use std::collections::BinaryHeap;

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
        self.walls[y as usize][x as usize]
    }
}

pub fn part1(lines: &[String]) -> String {
    // ok this current method doesn't care which direction we are facing for distance
    // so we might overwrite better solutions for 'short term' gains
    let turn_cost = 1000;
    let step_cost = 1;
    let d = import(lines);
    let max = d.walls.len() * d.walls[0].len();
    let end_index = d.yx_to_index(d.end);
    let mut pq: BinaryHeap<Vertex> = BinaryHeap::new();
    let mut dist = vec![usize::MAX; max];
    let mut dir = vec![Direction::Right; max];
    dist[d.yx_to_index(d.start)] = 0;
    pq.push(Vertex {
        index: d.yx_to_index(d.start),
        dist: 0,
    });
    while let Some(next) = pq.pop() {
        if next.index == end_index {
            return next.dist.to_string();
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
        let next_yx = d.index_to_yx(next.index);

        for (next_dist, direction) in next_dirs.iter() {
            let to = d.yx_to_index(direction.map_offset(next_yx));
            if d.get(to) {
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

    "NO ROUTE FOUND RIP".to_owned()
}

pub fn part2(lines: &[String]) -> String {
    let _ = import(lines);
    "".to_owned()
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
