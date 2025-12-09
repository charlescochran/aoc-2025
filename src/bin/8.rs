use itertools::Itertools;
use kiddo::{KdTree, SquaredEuclidean};
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::env;
use std::fs;

fn parse(input: &str) -> Vec<[f64; 3]> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn find_min_pairs(positions: &Vec<[f64; 3]>) -> Vec<(u64, u64, u64)> {
    let mut tree: KdTree<_, 3> = positions.into();
    let mut pairs = BTreeSet::new();
    let mut min_pairs = Vec::new();
    let mut prev_min_pairs = Vec::new();

    for n in 1..positions.len() {
        for (i, pos) in positions.iter().enumerate() {
            let neighbor = tree.nearest_n::<SquaredEuclidean>(pos, n + 1)[n];
            let idx1 = std::cmp::min(i as u64, neighbor.item);
            let idx2 = std::cmp::max(i as u64, neighbor.item);
            pairs.insert((neighbor.distance as u64, idx1, idx2));
        }
        min_pairs = pairs.iter().take(1000).cloned().collect();
        if !prev_min_pairs.is_empty() && min_pairs == prev_min_pairs {
            break;
        }
        std::mem::swap(&mut prev_min_pairs, &mut min_pairs);
    }
    min_pairs
}

fn create_circuits(min_pairs: Vec<(u64, u64, u64)>) -> Vec<HashSet<u64>> {
    let mut circuits: Vec<HashSet<u64>> = Vec::new();
    for (_distance, idx1, idx2) in min_pairs {
        let mut pair: HashSet<_> = [idx1, idx2].into();
        let mut i = 0;
        while i < circuits.len() {
            if circuits[i].is_disjoint(&pair) {
                i += 1;
                continue;
            }
            pair.extend(circuits.remove(i));
        }
        circuits.push(pair);
    }
    circuits
}

fn solve(positions: Vec<[f64; 3]>) -> (u64, u64) {
    let mut ans1: u64 = 0;
    let circuits = create_circuits(find_min_pairs(&positions));
    let mut ans1 = circuits
        .iter()
        .map(|c| c.len() as u64)
        .sorted()
        .rev()
        .take(3)
        .product();
    let mut ans2: u64 = 0;

    (ans1, ans2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let positions = parse(&input);

    let start = std::time::Instant::now();
    let (ans1, ans2) = solve(positions);
    let end = std::time::Instant::now();
    println!("Answer 1: {}", ans1);
    println!("Answer 2: {}", ans2);
    println!("Elapsed (both parts): {:?}", end - start);

    Ok(())
}
