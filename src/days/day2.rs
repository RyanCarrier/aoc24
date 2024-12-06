use crate::util::Problem;

pub const DAY2: Problem = Problem {
    day: 2,
    part1,
    part2,
    test_data: Some(test_data),
};

pub fn part1(lines: &[String]) -> String {
    solve2(lines, false)
}

pub fn part2(lines: &[String]) -> String {
    solve2(lines, true)
}
fn solve2(lines: &[String], part2: bool) -> String {
    lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_whitespace().map(|c| c.parse().unwrap()).collect())
        .fold(0, |acc, report| {
            acc + if safe(report, part2) { 1 } else { 0 }
        })
        .to_string()
}
fn safe(report: Vec<i32>, can_skip: bool) -> bool {
    //this is awful
    let len = report.len();
    let mut asc = report[0] < report[1];
    let asc2 = report[1] < report[2];
    if asc != asc2 && len > 3 {
        asc = report[2] < report[3];
    }
    let mut i = 1;
    let is_error = |a: i32, b: i32| -> bool {
        let diff = b - a;
        diff.abs() > 3 || (asc && diff <= 0) || (!asc && diff >= 0)
    };
    let mut can_skip = can_skip;
    let mut prev = report[0];
    while i < len {
        if is_error(prev, report[i]) {
            if can_skip {
                can_skip = false;
                if i == len - 1 {
                    //last char error
                    break;
                }
                if i == 1 {
                    if is_error(report[i], report[i + 1]) {
                        //second place error
                        i += 1;
                        continue;
                    }
                    if !is_error(report[i], report[i + 1]) {
                        //first place error
                        prev = report[i];
                        i += 1;
                        continue;
                    }
                    return false;
                }
                if !is_error(prev, report[i + 1]) {
                    // remove i
                    i += 1;
                    continue;
                }
                if !is_error(report[i - 2], report[i]) {
                    //remove i -1
                    prev = report[i];
                    i += 1;
                    continue;
                }
            }
            return false;
        }
        prev = report[i];
        i += 1;
    }
    true
}
fn solve(lines: &[String], part2: bool) -> String {
    let mut safe_reports = 0;
    lines.iter().filter(|l| !l.is_empty()).for_each(|l| {
        let report: Vec<i32> = l.split_whitespace().map(|c| c.parse().unwrap()).collect();
        let len = report.len();
        //jesus idk what this turned into
        // i should have just edited the vector lol
        let mut asc = report[0] < report[1];
        let asc2 = report[1] < report[2];
        if asc != asc2 && len > 4 {
            asc = report[2] < report[3];
        }
        safe_reports += 1;
        let mut error = false;
        let mut i = 0;
        let mut cmp_i = 1;
        'retry: while i < len - 1 {
            let diff = report[cmp_i] - report[i];
            if diff == 0 || diff.abs() > 3 || (asc && diff < 0) || (!asc && diff > 0) {
                if part2 && !error {
                    if cmp_i == len - 1 {
                        break;
                    }
                    error = true;
                    if i == 0 {
                        i = 1;
                        cmp_i = 2;
                    } else {
                        cmp_i = i + 2;
                    }
                    continue 'retry;
                }
                safe_reports -= 1;
                break;
            }
            i += cmp_i;
            cmp_i = i + 1;
        }
    });
    safe_reports.to_string()
}
pub fn test_data() -> &'static str {
    "28 25 26 25 24"
}
