use std::env;
use std::fs;

fn solve(input: &str, k: usize) -> u64 {
    let mut ans: u64 = 0;

    for line in input.lines() {
        let mut start = 0;
        let mut total = 0u64;
        for d in 0..k {
            let end = line.len() - k + d + 1;
            let (idx, c) = line[start..end]
                .bytes()  // bytes() instead of chars() for reversability
                .enumerate()
                .rev()
                .max_by_key(|&(_, c)| c)  // Find the max digit
                .unwrap();
            total = total * 10 + (c - b'0') as u64;
            start += idx + 1;
        }
        ans += total as u64;
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let start = std::time::Instant::now();
    let ans1 = solve(&input, 2);
    let ans2 = solve(&input, 12);
    let end = std::time::Instant::now();
    println!("Answer 1: {}", ans1);
    println!("Answer 2: {}", ans2);
    println!("Elapsed (both parts): {:?}", end - start);

    Ok(())
}
