use crate::util::{Problem, TRANSFORMS, TRANSFORMS_DIAG};

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};
//just incase i forget
const XMAS_LEN: isize = 4;
//lol
const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn xmas_scan(data: &[Vec<char>], (x, y): (usize, usize)) -> usize {
    let mut total = 0;
    for transform in TRANSFORMS.iter() {
        if xmas_scan_dir(data, (x, y), *transform, XMAS_LEN - 1) {
            total += 1;
        }
    }
    total
}
fn x_mas_scan(data: &[Vec<char>], (x, y): (usize, usize)) -> usize {
    let x = x as isize;
    let y = y as isize;
    if !in_range((x - 1, y - 1), data) || !in_range((x + 1, y + 1), data) {
        return 0;
    }

    // let mut total_hor = 0;
    // for transform in TRANSFORMS_HOR.iter() {
    //     if x_mas_scan_dir(data, (x, y), *transform) {
    //         total_hor += 1;
    //     }
    // }
    let mut total_diag = 0;
    for transform in TRANSFORMS_DIAG.iter() {
        if x_mas_scan_dir(data, (x, y), *transform) {
            total_diag += 1;
        }
    }
    // (total_hor / 2) + (total_diag / 2)
    // if total_hor == 2 || total_diag == 2 {
    //     1
    // } else {
    //     0
    // }
    //
    // HAHA it's only diagonal lol
    total_diag / 2
}
fn x_mas_scan_dir(data: &[Vec<char>], (x, y): (isize, isize), (dx, dy): (isize, isize)) -> bool {
    data[(y + dy) as usize][(x + dx) as usize] == 'M'
        && data[(y - dy) as usize][(x - dx) as usize] == 'S'
}

fn xmas_scan_dir(
    data: &[Vec<char>],
    (x, y): (usize, usize),
    (dx, dy): (isize, isize),
    len: isize,
) -> bool {
    let mut x = x as isize;
    let mut y = y as isize;
    if !in_range((x + (dx * len), y + (dy * len)), data) {
        return false;
    }
    for i in 1..XMAS_LEN {
        let c = XMAS[i as usize];
        x += dx;
        y += dy;
        if data[y as usize][x as usize] != c {
            return false;
        }
    }
    true
}
fn in_range((x, y): (isize, isize), data: &[Vec<char>]) -> bool {
    x >= 0 && y >= 0 && x < data[0].len() as isize && y < data.len() as isize
}

pub fn part1(lines: &[String]) -> String {
    let data = import(lines);
    //assume all line length same lol pls
    //get location of each x
    data.iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    if *c == 'X' {
                        Some(xmas_scan(&data, (x, y)))
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(lines: &[String]) -> String {
    let data = import(lines);
    //assume all line length same lol pls
    //get location of each x
    data.iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    if *c == 'A' {
                        Some(x_mas_scan(&data, (x, y)))
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}
pub fn test_data() -> &'static str {
    "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
}

fn import(lines: &[String]) -> Vec<Vec<char>> {
    lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}
