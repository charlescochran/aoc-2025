use std::collections::HashSet;
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once('-')
            .ok_or_else(|| format!("Invalid range format: {}", s))
            .and_then(|(start_str, end_str)| {
                let start = start_str.parse::<u64>().map_err(|e| e.to_string())?;
                let end = end_str.parse::<u64>().map_err(|e| e.to_string())?;
                if end < start {
                    Err(format!("Range end {} is less than start {}", end, start))
                } else {
                    Ok(Range { start, end })
                }
            })
    }
}

fn generate_invalids(range: &Range, d: usize) -> Vec<u64> {
    let mut invalids = Vec::new();
    let n_digits = range.start.to_string().len();
    // To avoid having to check every possibility, let's just increment a integer variable
    // representing the first section (repeated d times) of the potential ID.
    let mut i: u64 = if n_digits % d == 0 {
        // range.start is divisible by d, so just take the first n_digits/d digits
        range.start.to_string()[..n_digits / d].parse().unwrap()
    } else {
        // range.start is isn't divisible by d, so find the next greatest number with a number of
        // digits divisible by d
        ("1".to_string() + &"0".to_string().repeat(n_digits / d))
            .parse()
            .unwrap()
    };

    loop {
        // Generate an ID to check by duplicating i d times
        let id = i.to_string().repeat(d).parse::<u64>().unwrap();
        if id > range.end {
            break;
        }
        if id >= range.start {
            invalids.push(id);
        }
        i += 1;
    }

    invalids
}

fn solve(ranges: &[Range]) -> (u64, u64) {
    let mut ans1: u64 = 0;
    let mut ans2: u64 = 0;

    for range in ranges {
        let invalids1 = generate_invalids(range, 2);
        ans1 += invalids1.iter().sum::<u64>();

        let mut p2 = HashSet::new();
        // At most, d could be the length of the biggest number in the range (range.end)
        for d in 2..=range.end.to_string().len() {
            p2.extend(generate_invalids(range, d));
        }
        ans2 += p2.iter().sum::<u64>();
    }

    (ans1, ans2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let ranges: Vec<Range> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let start = std::time::Instant::now();
    let (ans1, ans2) = solve(&ranges);
    let end = std::time::Instant::now();
    println!("Answer 1: {}", ans1);
    println!("Answer 2: {}", ans2);
    println!("Elapsed (both parts): {:?}", end - start);

    Ok(())
}
