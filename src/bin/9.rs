use std::env;
use std::fs;

// fn get_bounds(points: &[[u64; 2]]) -> [u64; 4] {
//     let mut min_x = u64::MAX;
//     let mut max_x = u64::MIN;
//     let mut min_y = u64::MAX;
//     let mut max_y = u64::MIN;
//     for pos in points {
//         min_x = min_x.min(pos[0]);
//         max_x = max_x.max(pos[0]);
//         min_y = min_y.min(pos[1]);
//         max_y = max_y.max(pos[1]);
//     }
//     [min_x, max_x, min_y, max_y]
// }

fn biggest_area(dists: &[&[u64; 2]]) -> u64 {
    // Assumption: best point is within 10 closest points to each corner
    let search_n = 10;
    let mut max_area = 0;
    for [x1, y1] in &dists[..=search_n] {
        for [x2, y2] in &dists[dists.len() - search_n..] {
            let area = (x2.abs_diff(*x1) + 1) * (y2.abs_diff(*y1) + 1);
            max_area = max_area.max(area);
        }
    }
    max_area
}

fn solve1(points: Vec<[u64; 2]>) -> u64 {
    let max_x: u64 = points.iter().max_by_key(|[x, _y]| x).unwrap()[0];
    let mut top_left_dists: Vec<_> = points.iter().collect();
    let mut top_right_dists: Vec<_> = points.iter().collect();
    top_left_dists.sort_by_key(|[x, y]| x + y);
    top_right_dists.sort_by_key(|[x, y]| y + max_x - x);
    std::cmp::max(
        biggest_area(&top_left_dists),
        biggest_area(&top_right_dists),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let points: Vec<[u64; 2]> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            [x.parse().unwrap(), y.parse().unwrap()]
        })
        .collect();

    let start = std::time::Instant::now();
    let ans1 = solve1(points);
    let end = std::time::Instant::now();
    println!("Answer 1: {}", ans1);
    println!("Elapsed (both parts): {:?}", end - start);

    Ok(())
}
