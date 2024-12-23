use std::{collections::BinaryHeap, fs, path::Path, sync::Arc, time::Duration};

use reqwest::{cookie::Jar, Url};
#[derive(Clone, Copy)]
pub struct Problem {
    pub part1: fn(input: &[String]) -> String,
    pub part2: fn(input: &[String]) -> String,
    pub test_data: Option<fn() -> &'static str>,
}
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[allow(dead_code)]
impl Direction {
    pub fn get_directions() -> [Self; 4] {
        [Self::Up, Self::Right, Self::Down, Self::Left]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        Direction::get_directions().into_iter()
    }
    pub fn is_horizontal(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, Self::Up | Self::Down)
    }
    pub fn get_diagonal_offsets(&self) -> [(isize, isize); 2] {
        match self {
            Self::Up => [(-1, -1), (-1, 1)],
            Self::Down => [(1, -1), (1, 1)],
            Self::Left => [(-1, -1), (1, -1)],
            Self::Right => [(-1, 1), (1, 1)],
        }
    }
    pub fn get_offset(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
    pub fn map_offset(&self, pos: (isize, isize)) -> (isize, isize) {
        let offset = self.get_offset();
        (pos.0 + offset.0, pos.1 + offset.1)
    }
    pub fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    pub fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
    pub fn from_char(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
    pub fn from_arrow(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
    pub fn get_icon(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}
#[allow(dead_code)]
pub static TRANSFORMS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
#[allow(dead_code)]
pub static TRANSFORMS_HOR: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
#[allow(dead_code)]
pub static TRANSFORMS_DIAG: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

pub fn format_duration(d: Duration) -> String {
    if d.as_millis() > 1000 {
        return format!("{:.1}s", d.as_millis() as f64 / 1000_f64);
    }
    if d.as_micros() > 1000 {
        return format!("{:.1}ms", d.as_micros() as f64 / 1000_f64);
    }
    if d.as_nanos() > 1000 {
        return format!("{:.1}μs", d.as_nanos() as f64 / 1000_f64);
    }
    format!("{}ns", d.as_nanos())
}

pub fn get_input_data(year: usize, day: usize) -> Vec<String> {
    // get input data from aoc using cookie
    if !Path::new("./cookie.txt").exists() {
        panic!("cookie in a file called cookie.txt plzx");
    }
    let session_id = fs::read_to_string("./cookie.txt").unwrap();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day)
        .parse::<Url>()
        .unwrap();
    let cookie = format!("session={}", session_id);
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);
    let client = reqwest::blocking::ClientBuilder::new()
        .cookie_provider(Arc::new(jar))
        .build()
        .unwrap();

    let resp = client
        .get(url)
        // .header("cookie", format!("session={}", cookie))
        .send()
        .unwrap();
    if !resp.status().is_success() {
        panic!(
            "Failed to get input data, {:?}\n{:?}",
            resp.status(),
            resp.text().unwrap()
        );
    }
    let body = resp.text().unwrap();
    body.split('\n').map(|x| x.to_owned()).collect()
}

#[allow(dead_code)]
pub struct Graph {
    pub verticies: usize,
    pub arcs: Vec<Vec<VectorArc>>,
}
pub struct VectorArc {
    pub from: usize,
    pub to: usize,
    pub weight: usize,
}
#[allow(dead_code)]
impl VectorArc {
    pub fn new(from: usize, to: usize, weight: usize) -> Self {
        Self { from, to, weight }
    }
}
#[derive(Copy, Clone, Eq, Debug)]
#[allow(dead_code)]
pub struct Vertex {
    pub index: usize,
    pub dist: usize,
}
impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}
impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist).reverse()
        // other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[allow(dead_code)]
impl Graph {
    pub fn dijkstra(
        &self,
        from: usize,
        to: usize,
        weight_fn: Option<fn(usize, usize) -> usize>,
    ) -> usize {
        let mut dist = vec![usize::MAX; self.verticies];
        let mut pq: BinaryHeap<Vertex> = BinaryHeap::new();
        dist[from] = 0;
        pq.push(Vertex {
            index: from,
            dist: 0,
        });
        while let Some(next) = pq.pop() {
            println!("{:?}", next);
            if next.index == to {
                return next.dist;
            }
            for arc in self.arcs[next.index].iter() {
                let new_dist = if let Some(weight_fn) = weight_fn {
                    next.dist + weight_fn(arc.from, arc.to)
                } else {
                    next.dist + arc.weight
                };
                println!(
                    "{}=={}->{} newdist {} dist[] {}",
                    next.index, arc.from, arc.to, new_dist, dist[arc.to]
                );
                if new_dist < dist[arc.to] {
                    dist[arc.to] = new_dist;
                    pq.push(Vertex {
                        index: arc.to,
                        dist: new_dist,
                    });
                }
            }
            println!("{:?}", pq);
        }
        0
    }
}
//test for dijkstra
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let graph = Graph {
            verticies: 6,
            arcs: vec![
                vec![VectorArc::new(0, 1, 1), VectorArc::new(0, 2, 12)],
                vec![
                    VectorArc::new(1, 2, 9),
                    VectorArc::new(1, 3, 3),
                    VectorArc::new(1, 2, 1),
                ],
                vec![VectorArc::new(2, 4, 5)],
                vec![
                    VectorArc::new(3, 2, 4),
                    VectorArc::new(3, 4, 13),
                    VectorArc::new(3, 5, 15),
                ],
                vec![VectorArc::new(4, 5, 4)],
                vec![],
            ],
        };
        assert_eq!(graph.dijkstra(0, 5, None), 11);
        assert_eq!(graph.dijkstra(0, 2, None), 2);
    }
}
