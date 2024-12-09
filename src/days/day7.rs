use crate::util::Problem;

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};
struct Test {
    value: usize,
    params: Vec<usize>,
}
impl Test {
    fn is_true(&self) -> bool {
        self.is_true_from(1, self.params[0], self.value)
    }
    fn is_true_from(&self, start: usize, _lhs: usize, want: usize) -> bool {
        if _lhs > want {
            return false;
        }
        if start == self.params.len() - 1 {
            return _lhs + self.params[start] == self.value
                || _lhs * self.params[start] == self.value;
        }
        self.is_true_from(start + 1, _lhs + self.params[start], want)
            || self.is_true_from(start + 1, self.params[start] * _lhs, want)
    }
    fn is_true2(&self) -> bool {
        self.is_true_from2(1, self.params[0], self.value)
    }
    fn is_true_from2(&self, start: usize, _lhs: usize, want: usize) -> bool {
        if _lhs > want {
            return false;
        }
        let p = self.params[start];
        let options: [usize; 3] = [
            _lhs + p,
            _lhs * p,
            (_lhs.to_string() + &p.to_string()).parse().unwrap(),
        ];
        if start == self.params.len() - 1 {
            return options.iter().any(|o| *o == self.value);
        }
        options
            .iter()
            .any(|o| self.is_true_from2(start + 1, *o, want))
    }
}

pub fn part1(lines: &[String]) -> String {
    let tests = import(lines);
    tests
        .iter()
        .filter(|t| t.is_true())
        .map(|t| t.value)
        .sum::<usize>()
        .to_string()
}

pub fn part2(lines: &[String]) -> String {
    let tests = import(lines);
    tests
        .iter()
        .filter(|t| t.is_true2())
        .map(|t| t.value)
        .sum::<usize>()
        .to_string()
}
pub fn test_data() -> &'static str {
    "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
}

fn import(lines: &[String]) -> Vec<Test> {
    lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split(": ");
            let value = parts.next().unwrap().parse().unwrap();
            let operators = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|o| o.parse().unwrap())
                .collect();
            Test {
                value,
                params: operators,
            }
        })
        .collect()
}
