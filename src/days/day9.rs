use num::Integer;

use crate::util::Problem;

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};
struct Data {
    data: Vec<File>,
}

#[derive(Clone, Copy)]
struct File {
    id: usize,
    length: usize,
    has_moved: bool,
    // pos: usize,
}

impl File {
    fn is_gap(&self) -> bool {
        self.id == usize::MAX
    }
    fn gap(length: usize) -> Self {
        Self {
            id: usize::MAX,
            length,
            has_moved: true,
            // pos,
        }
    }
}
impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_gap() {
            write!(f, "{}", String::from_utf8(vec![b'.'; self.length]).unwrap())
        } else {
            write!(
                f,
                "{}",
                String::from_utf8(vec![(self.id as u8) + b'0'; self.length]).unwrap()
            )
        }
    }
}

impl Data {
    fn checksum(&self) -> usize {
        self.data
            .iter()
            .fold((0, 0), |(acc, pos), f| {
                let new_pos = pos + f.length;
                if f.is_gap() {
                    return (acc, new_pos);
                }
                (acc + ((pos..new_pos).sum::<usize>() * f.id), new_pos)
            })
            .0
    }
    fn fold(&mut self) {
        let mut i = 0;
        while i < self.data.len() - 1 {
            let j = self.data.len() - 1;
            if !self.data[i].is_gap() {
                i += 1;
                continue;
            }
            if self.data[j].is_gap() {
                self.data.remove(j);
                continue;
            }
            match self.data[i].length.cmp(&self.data[j].length) {
                std::cmp::Ordering::Equal => {
                    let d = self.data.remove(j);
                    self.data[i] = d;
                }
                std::cmp::Ordering::Less => {
                    //data bigger
                    let diff = self.data[j].length - self.data[i].length;
                    let mut d = self.data[j];
                    d.length = self.data[i].length;
                    self.data[i] = d;
                    self.data[j].length = diff;
                }
                std::cmp::Ordering::Greater => {
                    //gap bigger
                    let diff = self.data[i].length - self.data[j].length;
                    let mut new_gap = self.data[i];
                    new_gap.length = diff;
                    self.data[i] = self.data.remove(j);
                    self.data.insert(i + 1, new_gap);
                }
            }
        }
    }
    fn fold2(&mut self) {
        let mut j = self.data.len() - 1;
        // let mut dont_move = 0;
        while j > 0 {
            if self.data[j].has_moved {
                j -= 1;
                continue;
            }
            // println!("Find gap for id {}", self.data[j].id);
            let candidate = self.data[0..j]
                .iter()
                .position(|f| f.is_gap() && f.length >= self.data[j].length);
            self.data[j].has_moved = true;
            let i = match candidate {
                Some(k) => k,
                None => {
                    // println!("Coult not find gap for id {}", self.data[j].id);
                    if self.data[j].length == 1 {
                        return;
                    }
                    // dont_move += 2;
                    j -= 1;
                    continue;
                }
            };
            if self.data[i].length == self.data[j].length {
                self.data.swap(i, j);
            } else {
                //gap bigger
                let diff = self.data[i].length - self.data[j].length;
                self.data[i] = self.data[j];
                self.data[j].id = usize::MAX;
                self.data.insert(i + 1, File::gap(diff));
            }
            self.data[i].has_moved = true;
            self.data[j].has_moved = true;
            // dont_move += 1;
            while j < self.data.len() && self.data[j].is_gap() && self.data[j - 1].is_gap() {
                self.data[j - 1].length += self.data[j].length;
                self.data.remove(j);
            }
            // j = self.data.len() - 1 - dont_move;
            j = self.data.len() - 1;
        }
    }
}
impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().try_for_each(|d| write!(f, "{}", d))
    }
}

pub fn part1(lines: &[String]) -> String {
    let mut d = import(lines);
    d.fold();
    d.checksum().to_string()
}

pub fn part2(lines: &[String]) -> String {
    let mut d = import(lines);
    d.fold2();
    // println!("{}", d);
    d.checksum().to_string()
}
pub fn test_data() -> &'static str {
    "2333133121414131402"
}

fn import(lines: &[String]) -> Data {
    let lines: Vec<&String> = lines.iter().filter(|l| !l.is_empty()).collect();
    Data {
        data: lines[0]
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let length = ((c as u8) - b'0') as usize;
                if i.is_even() {
                    File {
                        id: i / 2,
                        length,
                        has_moved: false,
                    }
                } else {
                    File::gap(length)
                }
            })
            .collect(),
    }
}
