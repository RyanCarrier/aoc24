use std::collections::HashSet;

use crate::util::Problem;

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};
struct Data {
    max_yx: (isize, isize),
    antennas: Vec<(u8, (isize, isize))>,
}
impl Data {
    fn signals(&self) -> HashSet<u8> {
        self.antennas.iter().map(|(freq, _)| *freq).collect()
    }
    fn signal_groups(&self) -> Vec<Vec<&(u8, (isize, isize))>> {
        self.signals()
            .iter()
            .map(|freq| self.antennas.iter().filter(|(f, _)| f == freq).collect())
            .collect()
    }
    fn anti_ok(&self, node: (isize, isize)) -> bool {
        node.0 >= 0 && node.1 >= 0 && node.0 < self.max_yx.0 && node.1 < self.max_yx.1
    }

    fn anti_nodes(&self) -> Vec<(isize, isize)> {
        let mut anti_nodes: Vec<(isize, isize)> = Vec::new();
        for group in self.signal_groups() {
            for (i, (_, (ay, ax))) in group.iter().enumerate() {
                for (_, (by, bx)) in group.iter().skip(i + 1) {
                    let (dy, dx) = (by - ay, bx - ax);
                    let b_anti = (by + dy, bx + dx);
                    let a_anti = (ay - dy, ax - dx);
                    if self.anti_ok(b_anti) {
                        anti_nodes.push(b_anti);
                    }
                    if self.anti_ok(a_anti) {
                        anti_nodes.push(a_anti);
                    }
                }
            }
        }
        anti_nodes
            .into_iter()
            .collect::<HashSet<(isize, isize)>>()
            .into_iter()
            .collect()
    }
    fn anti_nodes2(&self) -> Vec<(isize, isize)> {
        let mut anti_nodes: Vec<(isize, isize)> = Vec::new();
        for group in self.signal_groups() {
            for (i, (_, (ay, ax))) in group.iter().enumerate() {
                for (_, (by, bx)) in group.iter().skip(i + 1) {
                    let (dy, dx) = (by - ay, bx - ax);
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let mut b_anti = (*by, *bx);
                    while self.anti_ok(b_anti) {
                        anti_nodes.push(b_anti);
                        b_anti = (b_anti.0 + dy, b_anti.1 + dx);
                    }
                    let mut a_anti = (*ay, *ax);
                    while self.anti_ok(a_anti) {
                        anti_nodes.push(a_anti);
                        a_anti = (a_anti.0 - dy, a_anti.1 - dx);
                    }
                }
            }
        }
        anti_nodes
            .into_iter()
            .collect::<HashSet<(isize, isize)>>()
            .into_iter()
            .collect()
    }
}

pub fn part1(lines: &[String]) -> String {
    let d = import(lines);
    // The signal only applies its nefarious effect at specific antinodes based on the resonant frequencies of the antennas.
    // In particular, an antinode occurs at any point that is perfectly in line with two antennas of the same frequency - but only when one of the antennas is twice as far away as the other.
    // This means that for any pair of antennas with the same frequency, there are two antinodes, one on either side of them.
    //
    // perfectly in line with two antennas (same), one is twice as far away as the other
    d.anti_nodes()
        .into_iter()
        .collect::<HashSet<(isize, isize)>>()
        .len()
        .to_string()
}

pub fn part2(lines: &[String]) -> String {
    let d = import(lines);
    d.anti_nodes2()
        .into_iter()
        .collect::<HashSet<(isize, isize)>>()
        .len()
        .to_string()
}
pub fn test_data() -> &'static str {
    "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
}

fn import(lines: &[String]) -> Data {
    let lines: Vec<&String> = lines.iter().filter(|l| !l.is_empty()).collect();
    Data {
        max_yx: (lines.len() as isize, lines[0].len() as isize),
        antennas: lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '.' {
                        None
                    } else {
                        Some((c as u8 - b'0', (y as isize, x as isize)))
                    }
                })
            })
            .collect(),
    }
}
