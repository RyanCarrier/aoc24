use std::ops::Mul;

use crate::util::Problem;
use nalgebra::{Matrix2, Matrix2x1};

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};

pub fn part1(lines: &[String]) -> String {
    let machines = import(lines);
    machines
        .filter_map(|m| m.solve_manual())
        .map(|(a, b)| 3 * a + b)
        .sum::<usize>()
        .to_string()
}

pub fn part2(lines: &[String]) -> String {
    let machines = import(lines);
    let extra = 10000000000000;
    machines
        .filter_map(|mut m| {
            m.prize = (m.prize.0 + extra, m.prize.1 + extra);
            m.solve_manual()
        })
        .map(|(a, b)| 3 * a + b)
        .sum::<usize>()
        .to_string()
}

struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}
impl Machine {
    fn solve_manual(&self) -> Option<(usize, usize)> {
        let (ax, ay) = (self.a.0 as isize, self.a.1 as isize);
        let (bx, by) = (self.b.0 as isize, self.b.1 as isize);
        let (px, py) = (self.prize.0 as isize, self.prize.1 as isize);
        let det = ax * by - bx * ay;
        if det == 0 {
            return None;
        }
        let a_pre_det = by * px - bx * py;
        let b_pre_det = -ay * px + ax * py;
        if a_pre_det % det == 0 && b_pre_det % det == 0 {
            Some(((a_pre_det / det) as usize, (b_pre_det / det) as usize))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn solve(&self) -> Option<(usize, usize)> {
        //ok we are figuring out how many multiples of a and b, not multiples of x and y...
        //so order of matrix is wrong, I've already made the manual one so not fixing this
        //one
        //INCORRECT MATRIX
        //[ax, ay]; [bx, by]
        //CORRECT
        //[ax,bx]; [ay, by]
        //
        //INCORRECT
        //inv = [[by, -ay,] [-bx, ax]] / det
        //CORRECT
        //inv = [[by, -bx,] [-ay, ax]] / det
        println!("A: {:?}, B: {:?}, Prize: {:?}", self.a, self.b, self.prize);
        let mat_a = Matrix2::new(
            self.a.0 as f64,
            self.a.1 as f64,
            self.b.0 as f64,
            self.b.1 as f64,
        );
        let inv_a = mat_a.try_inverse()?;
        println!("INV_A: {:?}", inv_a);
        let result = inv_a.mul(Matrix2x1::new(self.prize.0 as f64, self.prize.1 as f64));
        let (a, b) = (result.get(0).unwrap(), result.get(1).unwrap());
        // we could just test the function works with the whole numbers but eps check
        // instead yolo
        if (a.round() - a).abs() < 0.0001 && (b.round() - b).abs() < 0.0001 {
            println!("INTIES a: {}, b: {}", a, b);
            Some((a.round() as usize, b.round() as usize))
        } else {
            println!("a: {}, b: {}", a, b);
            None
        }
    }
}

fn import(lines: &[String]) -> impl Iterator<Item = Machine> + '_ {
    let lines = lines
        .iter()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&String>>();
    fn to_button(s: &str) -> (usize, usize) {
        let split = s.split_once(", ").unwrap();
        (
            split.0.split_once('+').unwrap().1.parse().unwrap(),
            split.1.split_once('+').unwrap().1.parse().unwrap(),
        )
    }
    (0..lines.len()).step_by(3).map(move |i| {
        let a = to_button(lines[i]);
        let b = to_button(lines[i + 1]);
        let split = lines[i + 2].split_once(", ").unwrap();
        let prize = (
            split.0.split_once('=').unwrap().1.parse().unwrap(),
            split.1.split_once('=').unwrap().1.parse().unwrap(),
        );
        Machine { a, b, prize }
    })
}
pub fn test_data() -> &'static str {
    "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
}
