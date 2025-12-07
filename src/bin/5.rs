use std::env;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    ID,
    START,
    END,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Num {
    value: u64,
    kind: Kind,
}

impl Num {
    fn new(s: &str, kind: Kind) -> Self {
        let value = s.parse::<u64>().expect("Invalid value");
        Self { value, kind }
    }
}

fn solve(nums: &[Num]) -> (u64, u64) {
    let mut ans1: u64 = 0;
    let mut ans2: u64 = 0;
    let mut prev: u64 = 0;
    let mut cnt: u32 = 0;

    for num in nums.iter() {
        match num.kind {
            Kind::START => {
                if cnt == 0 {
                    prev = num.value;
                }
                cnt += 1;
            }
            Kind::END => {
                cnt -= 1;
                if cnt == 0 {
                    ans2 += num.value - prev + 1;
                }
            }
            Kind::ID => {
                if cnt > 0 {
                    ans1 += 1;
                }
            }
        }
    }

    (ans1, ans2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let (range_str, ids_str) = input.trim().split_once("\n\n").expect("Invalid input");
    let mut nums = Vec::new();

    for line in range_str.lines() {
        let (start_str, end_str) = line.split_once('-').expect("Invalid range line");
        nums.push(Num::new(start_str, Kind::START));
        nums.push(Num::new(end_str, Kind::END));
    }
    for id_str in ids_str.lines() {
        nums.push(Num::new(id_str, Kind::ID));
    }
    nums.sort();

    let start = std::time::Instant::now();
    let (ans1, ans2) = solve(&nums);
    let end = std::time::Instant::now();
    println!("Answer 1: {}", ans1);
    println!("Answer 2: {}", ans2);
    println!("Elapsed (both parts): {:?}", end - start);

    Ok(())
}
