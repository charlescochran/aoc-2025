use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Turn {
    L(i32),
    R(i32),
}

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, num) = s.split_at(1);
        let value = num.parse::<i32>().map_err(|e| e.to_string())?;

        match dir {
            "L" => Ok(Turn::L(value)),
            "R" => Ok(Turn::R(value)),
            _ => Err(format!("Invalid direction '{}'", dir)),
        }
    }
}

impl Turn {
    fn apply(&self, counter: i32) -> (i32, i32, i32) {
        let delta = match self {
            Turn::L(v) => -*v,
            Turn::R(v) => *v,
        };
        let new = (counter + delta).rem_euclid(100);
        let p1: i32 = if new == 0 { 1 } else { 0 };
        let mut p2 = delta.abs().div_euclid(100);
        if (counter != 0 && (new - counter).signum() != delta.signum()) || new == 0 {
            p2 += 1;
        }

        (new, p1, p2)
    }
}

fn solve(turns: &[Turn]) -> (i32, i32) {
    let mut counter = 50;
    let mut ans1 = 0;
    let mut ans2 = 0;

    for turn in turns {
        let (new, p1, p2) = turn.apply(counter);
        counter = new;
        ans1 += p1;
        ans2 += p2;
    }

    (ans1, ans2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let turns: Vec<Turn> = input.lines().map(|line| line.parse().unwrap()).collect();

    let start = std::time::Instant::now();
    let (ans1, ans2) = solve(&turns);
    let end = std::time::Instant::now();
    println!("Answer 1: {}", ans1);
    println!("Answer 2: {}", ans2);
    println!("Elapsed (both parts): {:?}", end - start);

    Ok(())
}
