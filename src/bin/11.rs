use std::collections::HashMap;
use std::env;
use std::fs;

type Devices = HashMap<String, Vec<String>>;

fn explore<'a>(
    devices: &'a Devices,
    cache: &mut HashMap<(&'a str, bool, bool), u64>,
    current: &'a str,
    goal: &str,
    mut dac: bool,
    mut fft: bool,
    part2: bool,
) -> u64 {
    dac |= current == "dac";
    fft |= current == "fft";

    if current == goal {
        if part2 {
            return if dac && fft { 1 } else { 0 };
        }
        return 1;
    }

    if let Some(&count) = cache.get(&(current, dac, fft)) {
        return count;
    }

    let mut count = 0;
    if let Some(neighbors) = devices.get(current) {
        for next in neighbors {
            count += explore(devices, cache, next, goal, dac, fft, part2);
        }
    }

    cache.insert((current, dac, fft), count);

    count
}

fn parse(input: &str) -> Devices {
    let mut devices = Devices::new();
    for line in input.lines() {
        let (name, next) = line.split_once(" ").unwrap();
        let name = name.strip_suffix(":").unwrap().into();
        let next = next.split(" ").map(String::from).collect();
        devices.insert(name, next);
    }
    devices
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let devices = parse(&input);

    let start = std::time::Instant::now();
    let ans1 = explore(&devices, &mut [].into(), "you", "out", false, false, false);
    let ans2 = explore(&devices, &mut [].into(), "svr", "out", false, false, true);
    let end = std::time::Instant::now();
    println!("Answer 1: {}", ans1);
    println!("Answer 2: {}", ans2);
    println!("Elapsed (both parts): {:?}", end - start);

    Ok(())
}
