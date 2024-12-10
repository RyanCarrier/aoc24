use ansi_term::Style;
use clap::Parser;
use std::time::{Duration, Instant};
use util::Problem;

pub mod days;
mod util;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    day: usize,
    #[arg(short, long, default_value_t = 0)]
    part: usize,
    #[arg(short, long)]
    test: bool,
    #[arg(short, long)]
    benchmark: bool,
    #[arg(short, long, default_value_t = 100)]
    iterations: usize,
}
const YEAR: usize = 2024;
const DAYS: [Problem; 9] = [
    days::day1::PROBLEM,
    days::day2::PROBLEM,
    days::day3::PROBLEM,
    days::day4::PROBLEM,
    days::day5::PROBLEM,
    days::day6::PROBLEM,
    days::day7::PROBLEM,
    days::day8::PROBLEM,
    days::day9::PROBLEM,
];

fn main() {
    let args = Args::parse();
    if args.benchmark {
        benchmark(args);
        return;
    }
    let problems = DAYS.to_vec();
    if args.day == 0 {
        //just assume
        for day in 1..=problems.len() {
            run_specific(day, &problems[day - 1], &args);
        }
        return;
    }
    run_specific(args.day, &problems[args.day - 1], &args);
}

fn run_specific(day: usize, problem: &Problem, args: &Args) {
    println!(
        "{}",
        Style::new()
            .bold()
            .paint("=== Day ".to_owned() + &day.to_string() + " ==="),
    );
    let input = if args.test {
        (problem
            .test_data
            .expect("Asked for test data, but there was none set"))()
        .split('\n')
        .map(|x| x.to_owned())
        .collect()
    } else {
        util::get_input_data(YEAR, day)
    };
    let start = Instant::now();
    if args.part == 0 || args.part == 1 {
        print_result(day, 1, args.test, (problem.part1)(&input));
    }
    let part1_duration = start.elapsed();
    if args.part == 0 || args.part == 2 {
        print_result(day, 2, args.test, (problem.part2)(&input));
    }
    let total_duration = start.elapsed();
    println!(
        "Completed in {}\t(p1:{}, p2:{})",
        util::format_duration(total_duration),
        util::format_duration(part1_duration),
        util::format_duration(total_duration - part1_duration)
    );
}
fn print_result(day: usize, part: usize, test: bool, result: String) {
    println!(
        "day{}part{}{}:\t{}",
        day,
        part,
        if test { "-TEST" } else { "" },
        result
    );
}

fn benchmark(args: Args) {
    let runs: usize = args.iterations;
    let problems = DAYS.to_vec();
    let range = if args.day == 0 {
        0..problems.len()
    } else {
        (args.day - 1)..(args.day)
    };
    let max = if args.day == 0 {
        problems.len()
    } else {
        args.day
    };
    let mut data = vec![vec![String::new()]; max];
    print!("Getting input data... ");
    for day in range.clone() {
        print!("{}... ", day + 1);
        data[day] = util::get_input_data(YEAR, day + 1);
    }
    println!(
        "{}, {} iterations",
        Style::new().bold().paint("Day durations"),
        runs
    );
    println!("Day\t\tPart1\tPart2\tTotal");
    let mut part1_durations = vec![Duration::default(); max];
    let mut part2_durations = vec![Duration::default(); max];
    let mut total_duration = Duration::default();
    for day in range.clone() {
        let start = Instant::now();
        for _ in 0..runs {
            (problems[day].part1)(&data[day]);
        }
        part1_durations[day] = start.elapsed().div_f64(runs as f64);
        let start = Instant::now();
        for _ in runs..(runs * 2) {
            (problems[day].part2)(&data[day]);
        }
        part2_durations[day] = start.elapsed().div_f64(runs as f64);
        println!(
            "Day {}:\t\t{}\t{}\t{}",
            day + 1,
            util::format_duration(part1_durations[day]),
            util::format_duration(part2_durations[day]),
            Style::new().bold().paint(util::format_duration(
                part1_durations[day] + part2_durations[day]
            ))
        );
        total_duration += part1_durations[day] + part2_durations[day];
    }
    println!(
        "{}",
        Style::new()
            .bold()
            .paint(format!("Total:\t{}", util::format_duration(total_duration)))
    );
}
