use std::env;
use std::fs;

#[derive(Debug, Clone)]
enum Kind {
    ADD,
    MULTIPLY,
}

#[derive(Debug, Clone)]
struct Problem {
    value: Vec<u64>,
    kind: Kind,
}

fn solve(problems: &[Problem]) -> u64 {
    let mut ans: u64 = 0;
    for problem in problems.iter() {
        match problem.kind {
            Kind::ADD => {
                ans += problem.value.iter().sum::<u64>();
            },
            Kind::MULTIPLY => {
                ans += problem.value.iter().product::<u64>();
            },
        }
    }
    ans
}

fn populate_nums1(problems: &mut [Problem], num_strs: &[&str]) {
    let nums: Vec<Vec<u64>> = num_strs
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    for row in nums.iter() {
        for (i, &num) in row.iter().enumerate() {
            problems[i].value.push(num);
        }
    }
}

fn populate_nums2(problems: &mut [Problem], num_strs: &[&str]) {
    let mut i = 0;
    let chars: Vec<Vec<char>> = num_strs.iter().map(|line| line.chars().collect()).collect();
    for c in 0..chars[0].len() {
        let mut col = Vec::new();
        for r in 0..chars.len() {
            if chars[r][c] == ' ' {
                continue;
            }
            col.push(chars[r][c]);
        }
        if col.is_empty() {
            i += 1;
            continue;
        }
        let num: u64 = col.iter().collect::<String>().parse().unwrap();
        problems[i].value.push(num);
    }
}

fn parse_input(input: &str) -> (Vec<Problem>, Vec<Problem>) {
    let lines: Vec<&str> = input.lines().collect();
    let (symbol_str, num_strs) = lines.split_last().unwrap();

    let mut problems1: Vec<Problem> = symbol_str
        .split_whitespace()
        .map(|symbol| match symbol {
            "+" => Problem {
                kind: Kind::ADD,
                value: Vec::new(),
            },
            "*" => Problem {
                kind: Kind::MULTIPLY,
                value: Vec::new(),
            },
            _ => panic!("Unexpected symbol: {}", symbol),
        })
        .collect();
    let mut problems2 = problems1.clone();

    populate_nums1(&mut problems1, &num_strs);
    populate_nums2(&mut problems2, &num_strs);

    (problems1, problems2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let (problems1, problems2) = parse_input(&input);

    let start = std::time::Instant::now();
    let ans1 = solve(&problems1);
    let ans2 = solve(&problems2);
    let end = std::time::Instant::now();
    println!("Answer 1: {}", ans1);
    println!("Answer 2: {}", ans2);
    println!("Elapsed (both parts): {:?}", end - start);

    Ok(())
}
