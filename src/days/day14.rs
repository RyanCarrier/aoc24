use regex::Regex;

use crate::util::Problem;

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};

pub fn part1(lines: &[String]) -> String {
    let guards = import(lines);
    let (x_max, y_max) = if guards.len() == 12 {
        //we are in test env
        (11, 7)
    } else {
        (101, 103)
    };
    let (x_mid, y_mid) = (x_max / 2, y_max / 2);
    let seconds = 100;
    guards
        .into_iter()
        .map(|g| {
            (
                (g[0] + g[2] * seconds).rem_euclid(x_max),
                (g[1] + g[3] * seconds).rem_euclid(y_max),
            )
        })
        .fold([0, 0, 0, 0], |mut acc, (x, y)| {
            if x == x_mid || y == y_mid {
                return acc;
            }
            let index = if x < x_mid { 0 } else { 1 } + if y < y_mid { 0 } else { 2 };
            acc[index] += 1;
            acc
        })
        .into_iter()
        .product::<usize>()
        .to_string()
}

pub fn part2(lines: &[String]) -> String {
    let mut guards = import(lines);
    let (x_max, y_max) = (101, 103);
    let mut xs;
    let mut ys;
    for i in 1..=usize::MAX {
        xs = vec![0; x_max as usize];
        ys = vec![0; y_max as usize];
        for g in guards.iter_mut() {
            g[0] = (g[0] + g[2]).rem_euclid(x_max);
            g[1] = (g[1] + g[3]).rem_euclid(y_max);
            xs[g[0] as usize] += 1;
            ys[g[1] as usize] += 1;
        }
        // 31
        let x_count = xs.iter().filter(|&&count| count > 30).count();
        let y_count = ys.iter().filter(|&&count| count > 30).count();
        if y_count > 1 && x_count > 1 {
            return i.to_string();
        }
    }
    "CHRISTMAS IS CANCELLED".to_string()
}

#[allow(dead_code)]
fn part2_manual(lines: &[String]) -> String {
    let mut guards = import(lines);
    let (x_max, y_max) = (101, 103);
    let mut xs;
    let mut ys;
    print_guards(&guards, (x_max, y_max), 0);
    for i in 1..=10000 {
        xs = vec![0; x_max as usize];
        ys = vec![0; y_max as usize];
        for g in guards.iter_mut() {
            g[0] = (g[0] + g[2]).rem_euclid(x_max);
            g[1] = (g[1] + g[3]).rem_euclid(y_max);
            xs[g[0] as usize] += 1;
            ys[g[1] as usize] += 1;
        }
        let x_count = xs.iter().filter(|&&count| count > 30).count();
        let y_count = ys.iter().filter(|&&count| count > 30).count();
        if y_count > 1 && x_count > 1 {
            print_guards(&guards, (x_max, y_max), i);
            std::thread::sleep(std::time::Duration::from_millis(5000));
        }

        // got to 5000 manually and still hadnt' seen the picture so...
        //
        // 39
        // 140
        // tried checking just the common interval where there are clusters
        // vertically (there were horizontal clusters too and I'm sure this
        // period would also have worked for them)
        //
        // print_guards(&guards, (x_max, y_max), i);
        // std::thread::sleep(std::time::Duration::from_millis(10));
        //
        // if i > 39 && (i - 39) % 101 == 0 {
        //     print_guards(&guards, (x_max, y_max), i);
        //     std::thread::sleep(std::time::Duration::from_millis(500));
        // }
        // if i == 7412 {
        //     print_guards(&guards, (x_max, y_max), i);
        //     //sleep for 0.5s
        //     std::thread::sleep(std::time::Duration::from_millis(5000));
        // }
    }

    "".to_string()
}
fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
fn print_guards(guards: &[[isize; 4]], (x_max, y_max): (isize, isize), tick: usize) {
    clear_screen();
    println!("tick {}", tick);
    for y in 0..y_max {
        for x in 0..x_max {
            if guards.iter().any(|g| g[0] == x && g[1] == y) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
pub fn test_data() -> &'static str {
    "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
}

fn import(lines: &[String]) -> Vec<[isize; 4]> {
    let r = Regex::new(r"[p|v]=").unwrap();
    lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let cleaned_line = r.replace_all(line, "").replace(' ', ",");
            let mut num_iter = cleaned_line.split(',');
            [
                num_iter.next().unwrap().parse().unwrap(),
                num_iter.next().unwrap().parse().unwrap(),
                num_iter.next().unwrap().parse().unwrap(),
                num_iter.next().unwrap().parse().unwrap(),
            ]
        })
        .collect()
}
