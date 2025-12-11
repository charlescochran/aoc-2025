use disjoint::DisjointSet;
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

fn solve(positions: Vec<[f64; 3]>) -> (u64, u64) {
    // Used to quickly perform nearest neighbor searches
    let tree: KdTree<_, 3> = (&positions).into();
    // Pairs, sorted by distance
    let mut distances = BTreeSet::new();
    // Unique pairs (needed for part 1)
    let mut pairs = HashSet::new();
    // Pairs fround previously
    let mut prev = Vec::new();

    let mut ans1 = 0;
    let mut prev_diff_idx = 0;

    let mut circuits = DisjointSet::with_len(positions.len());
    // Find the nth nearest neighbor from each point
    for n in 1..positions.len() {
        for (i, pos) in positions.iter().enumerate() {
            let neighbor = tree.nearest_n::<SquaredEuclidean>(pos, n + 1)[n];
            distances.insert((neighbor.distance as u64, [i, neighbor.item as usize]));
        }
        // diff_idx is the index of the first pair that differs since last round (when n was one
        // smaller). The slice from prev_diff_idx to diff_idx have now been confirmed as minimum
        // distance pairs, so we can add them our circuit.
        let new: Vec<(u64, [usize; 2])> = distances.iter().cloned().collect();
        if let Some(diff_idx) = new.iter().zip(&prev).position(|(a, b)| a != b) {
            for (_distance, [idx1, idx2]) in &new[prev_diff_idx..diff_idx] {
                // Part 1: calculate answer after first 1000 pairs (not counting duplicates)
                pairs.insert([std::cmp::min(*idx1, *idx2), std::cmp::max(*idx1, *idx2)]);
                if pairs.len() == 1000 {
                    ans1 = circuits
                        .sets()
                        .iter()
                        .map(|c| c.len() as u64)
                        .sorted()
                        .rev()
                        .take(3)
                        .product();
                }

                // Part 2: stop once all boxes have been joined
                circuits.join(*idx1, *idx2);
                if (1..positions.len()).all(|i| circuits.is_joined(0, i)) {
                    let ans2 = (positions[*idx1][0] * positions[*idx2][0]) as u64;
                    return (ans1, ans2);
                }
            }
            prev_diff_idx = diff_idx;
        }
        prev = new;
    }
    unreachable!();
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
