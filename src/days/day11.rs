use std::collections::HashMap;

use num::Integer;

use crate::util::Problem;

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};
struct Data {
    memo: HashMap<(usize, usize), Vec<usize>>,
}
impl Data {
    fn step_all(
        &mut self,
        stone_freq: &HashMap<usize, usize>,
        steps: usize,
    ) -> HashMap<usize, usize> {
        let steps_per_leap = 5;
        if steps > steps_per_leap {
            //stops each step from having to return too large of an array
            // just chose 5 cause it seems reasonable and very small arrays
            // higher probably better cause hashmaps will be slower
            //
            // actually after quick testing 5 seems to be the best lol
            let a = self.step_all(stone_freq, steps_per_leap);
            return self.step_all(&a, steps - steps_per_leap);
        }
        stone_freq
            .iter()
            .fold(HashMap::new(), |mut acc, (x, freq)| {
                self.step(*x, steps).iter().for_each(|y| {
                    *acc.entry(*y).or_insert(0) += freq;
                });
                acc
            })
    }
    fn step(&mut self, x: usize, steps: usize) -> Vec<usize> {
        if steps == 0 {
            return vec![x];
        }
        if self.memo.contains_key(&(x, steps)) {
            return self.memo.get(&(x, steps)).unwrap().to_vec();
        }
        let result = if x == 0 {
            self.step(1, steps - 1)
        } else {
            let l = x.to_string().len();
            if l.is_even() {
                let m = 10_usize.pow(l as u32 / 2);
                let lhs = x / m;
                [self.step(lhs, steps - 1), self.step(x - lhs * m, steps - 1)].concat()
            } else {
                self.step(x * 2024, steps - 1)
            }
        };
        // if x < 5000 {
        // don't overfill with used once data
        //
        // disregard it runs fine with it without it
        self.memo.insert((x, steps), result.clone());
        // }
        result
    }
}

pub fn part1(lines: &[String]) -> String {
    let (initial, mut d) = import(lines);
    d.step_all(&initial, 25)
        .iter()
        .fold(0, |acc, (_, v)| acc + v)
        .to_string()
}

pub fn part2(lines: &[String]) -> String {
    let (initial, mut d) = import(lines);
    d.step_all(&initial, 75)
        .iter()
        .fold(0, |acc, (_, v)| acc + v)
        .to_string()
}
pub fn test_data() -> &'static str {
    // "0"
    "125 17"
}

fn import(lines: &[String]) -> (HashMap<usize, usize>, Data) {
    (
        lines
            .iter()
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .fold(HashMap::new(), |mut acc, x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            }),
        Data {
            memo: HashMap::new(),
        },
    )
}
