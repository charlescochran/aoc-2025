use std::collections::HashMap;
use std::env;
use std::fs;

fn solve(chars: &[Vec<char>]) -> (u64, u64) {
    let mut ans1: u64 = 0;
    let mut ans2: u64 = 1;
    let mut visited = HashMap::new();
    let start_x = chars[0]
        .iter()
        .position(|&c| c == 'S')
        .expect("Start not found");
    visited.insert((0, start_x), 1);

    while !visited.is_empty() {
        let mut new = HashMap::new();
        for ((r, c), &n) in visited.clone().iter() {
            // Stop this particle if it's at the bottom
            if r + 1 >= chars.len() {
                continue;
            }
            if chars[r + 1][*c] == '^' {
                ans1 += 1;
                ans2 += n;
                // Splitter: replace this particle with one to the left and one to the right
                if *c > 0 {
                    *new.entry((r + 1, c - 1)).or_insert(0) += n;
                }
                if *c + 1 < chars[0].len() {
                    *new.entry((r + 1, c + 1)).or_insert(0) += n;
                }
            } else {
                // No splitter, just continue down
                *new.entry((r + 1, *c)).or_insert(0) += n;
            }
        }
        visited = new;
    }

    (ans1, ans2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = std::time::Instant::now();
    let (ans1, ans2) = solve(&chars);
    let end = std::time::Instant::now();
    println!("Answer 1: {}", ans1);
    println!("Answer 2: {}", ans2);
    println!("Elapsed (both parts): {:?}", end - start);

    Ok(())
}
