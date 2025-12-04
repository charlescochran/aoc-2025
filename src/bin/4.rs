use std::env;
use std::fs;

const DIRECTIONS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn check(grid: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    let mut count = 0;
    for (dir_r, dir_c) in DIRECTIONS {
        if let Some(row) = grid.get((r as i32 + dir_r) as usize) {
            if let Some(&cell) = row.get((c as i32 + dir_c) as usize) {
                if cell == '@' {
                    count += 1;
                    if count > 3 {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn count_all(grid: &mut Vec<Vec<char>>, remove: bool) -> u64 {
    let mut count: u64 = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] != '@' {
                continue;
            }
            if check(&grid, r, c) {
                if remove {
                    grid[r][c] = '.';
                }
                count += 1;
            }
        }
    }

    count
}

fn solve(input: &str) -> (u64, u64) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let ans1 = count_all(&mut grid, false);
    let mut ans2: u64 = 0;
    loop {
        let new = count_all(&mut grid, true);
        if new == 0 {
            break;
        }
        ans2 += new;
    }

    (ans1, ans2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;

    let start = std::time::Instant::now();
    let (ans1, ans2) = solve(&input);
    let end = std::time::Instant::now();
    println!("Answer 1: {}", ans1);
    println!("Answer 2: {}", ans2);
    println!("Elapsed (both parts): {:?}", end - start);

    Ok(())
}
